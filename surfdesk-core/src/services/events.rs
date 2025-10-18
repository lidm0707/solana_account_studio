//! # Events Service Module
//!
//! This module provides an event system for the SurfDesk application.
//! It handles inter-component communication, event dispatching, and
//! subscription management across all platforms using Dioxus signals
//! for single-threaded compatibility.

use crate::error::{Result, SurfDeskError};
use serde_json::json;
use std::collections::HashMap;
use tokio::sync::broadcast;

/// Event service for managing application-wide events
#[derive(Debug, Clone)]
pub struct EventService {
    /// Event channels for different event types
    channels: HashMap<String, broadcast::Sender<Event>>,
    /// Event subscriptions
    subscriptions: HashMap<String, Vec<SubscriptionInfo>>,
    /// Event history
    history: Vec<EventHistoryEntry>,
    /// Maximum history size
    max_history_size: usize,
}

impl EventService {
    /// Create a new event service
    pub fn new() -> Result<Self> {
        let mut service = Self {
            channels: HashMap::new(),
            subscriptions: HashMap::new(),
            history: Vec::new(),
            max_history_size: 1000,
        };

        // Initialize default channels
        let default_channels = vec![
            "app.startup",
            "app.shutdown",
            "project.created",
            "project.updated",
            "project.deleted",
            "environment.created",
            "environment.updated",
            "environment.deleted",
            "account.updated",
            "transaction.sent",
            "transaction.confirmed",
            "transaction.failed",
            "solana.connected",
            "solana.disconnected",
            "ui.theme_changed",
            "ui.notification",
            "error.occurred",
        ];

        for channel_name in default_channels {
            if let Err(e) = service.create_channel_sync(channel_name) {
                log::warn!("Failed to create channel {}: {}", channel_name, e);
            }
        }

        log::info!("Event service initialized");
        Ok(service)
    }

    /// Create a new event channel (synchronous)
    fn create_channel_sync(&mut self, name: &str) -> Result<()> {
        if !self.channels.contains_key(name) {
            let (sender, _) = broadcast::channel(1000);
            self.channels.insert(name.to_string(), sender);
            log::debug!("Created event channel: {}", name);
        }
        Ok(())
    }

    /// Create a new event channel
    pub async fn create_channel(&mut self, name: &str) -> Result<()> {
        self.create_channel_sync(name)
    }

    /// Emit an event
    pub async fn emit(&mut self, event: Event) -> Result<()> {
        let channel_name = &event.event_type;

        // Get or create channel
        if !self.channels.contains_key(channel_name) {
            self.create_channel_sync(channel_name)?;
        }

        // Send event
        if let Some(sender) = self.channels.get(channel_name) {
            match sender.send(event.clone()) {
                Ok(count) => {
                    log::debug!("Event {} sent to {} subscribers", channel_name, count);
                }
                Err(_) => {
                    log::warn!("No active subscribers for event: {}", channel_name);
                }
            }
        }

        // Add to history
        self.add_to_history_sync(event.clone());

        // Log event
        log::debug!("Event emitted: {} - {}", channel_name, event.summary());

        Ok(())
    }

    /// Subscribe to an event type
    pub async fn subscribe(&mut self, event_type: &str) -> Result<broadcast::Receiver<Event>> {
        if let Some(sender) = self.channels.get(event_type) {
            let receiver = sender.subscribe();

            // Track subscription
            let subscription_info = SubscriptionInfo {
                id: uuid::Uuid::new_v4().to_string(),
                event_type: event_type.to_string(),
                created_at: chrono::Utc::now(),
            };

            self.subscriptions
                .entry(event_type.to_string())
                .or_insert_with(Vec::new)
                .push(subscription_info);

            log::debug!("New subscription to event type: {}", event_type);
            Ok(receiver)
        } else {
            Err(SurfDeskError::validation(format!(
                "Unknown event type: {}",
                event_type
            )))
        }
    }

    /// Subscribe to multiple event types
    pub async fn subscribe_multiple(
        &mut self,
        event_types: Vec<String>,
    ) -> Result<Vec<broadcast::Receiver<Event>>> {
        let mut receivers = Vec::new();

        for event_type in event_types {
            receivers.push(self.subscribe(&event_type).await?);
        }

        Ok(receivers)
    }

    /// Unsubscribe from an event type
    pub async fn unsubscribe(&mut self, subscription_id: &str) -> Result<()> {
        for (_event_type, subs) in self.subscriptions.iter_mut() {
            subs.retain(|sub| sub.id != subscription_id);
        }

        log::debug!("Subscription removed: {}", subscription_id);
        Ok(())
    }

    /// Add event to history (synchronous)
    fn add_to_history_sync(&mut self, event: Event) {
        self.history.push(EventHistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            event,
            timestamp: chrono::Utc::now(),
        });

        // Trim history if needed
        if self.history.len() > self.max_history_size {
            self.history.remove(0);
        }
    }

    /// Get event history
    pub async fn get_history(&self, limit: Option<usize>) -> Vec<EventHistoryEntry> {
        if let Some(limit) = limit {
            let start = if self.history.len() > limit {
                self.history.len() - limit
            } else {
                0
            };
            self.history[start..].to_vec()
        } else {
            self.history.clone()
        }
    }

    /// Get events by type
    pub async fn get_events_by_type(
        &self,
        event_type: &str,
        limit: Option<usize>,
    ) -> Vec<EventHistoryEntry> {
        let history = self.get_history(limit).await;
        history
            .into_iter()
            .filter(|entry| entry.event.event_type == event_type)
            .collect()
    }

    /// Get subscription information
    pub async fn get_subscriptions(&self) -> HashMap<String, Vec<SubscriptionInfo>> {
        self.subscriptions.clone()
    }

    /// Get channel information
    pub async fn get_channels(&self) -> Vec<String> {
        self.channels.keys().cloned().collect()
    }

    /// Clear event history
    pub async fn clear_history(&mut self) -> Result<()> {
        self.history.clear();
        log::info!("Event history cleared");
        Ok(())
    }

    /// Shutdown the event service
    pub async fn shutdown(&mut self) -> Result<()> {
        self.channels.clear();
        self.subscriptions.clear();
        self.history.clear();

        log::info!("Event service shutdown");
        Ok(())
    }
}

/// Event structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Event {
    /// Event type identifier
    pub event_type: String,
    /// Event data
    pub data: serde_json::Value,
    /// Event source
    pub source: String,
    /// Event timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Event ID
    pub id: String,
}

impl Event {
    /// Create a new event
    pub fn new(event_type: String, data: serde_json::Value, source: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            event_type,
            data,
            source,
        }
    }

    /// Get a summary of the event
    pub fn summary(&self) -> String {
        format!(
            "{} from {} at {}",
            self.event_type, self.source, self.timestamp
        )
    }

    /// Get event data as specific type
    pub fn data_as<T>(&self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_value(self.data.clone())
            .map_err(|e| SurfDeskError::Serialization(e.to_string()))
    }
}

/// Event history entry
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventHistoryEntry {
    /// Entry ID
    pub id: String,
    /// The event
    pub event: Event,
    /// When the event was recorded
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Subscription information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionInfo {
    /// Subscription ID
    pub id: String,
    /// Event type being subscribed to
    pub event_type: String,
    /// When the subscription was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Event builder for creating events conveniently
pub struct EventBuilder {
    event_type: String,
    data: serde_json::Value,
    source: String,
}

impl EventBuilder {
    /// Create a new event builder
    pub fn new(event_type: impl Into<String>) -> Self {
        Self {
            event_type: event_type.into(),
            data: serde_json::Value::Null,
            source: "unknown".to_string(),
        }
    }

    /// Set event data
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = data;
        self
    }

    /// Set event source
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    /// Build the event
    pub fn build(self) -> Event {
        Event::new(self.event_type, self.data, self.source)
    }
}

/// Create a project created event
pub fn project_created(project_id: &str, project_name: &str, source: &str) -> Event {
    EventBuilder::new("project.created")
        .data(json!({
            "project_id": project_id,
            "project_name": project_name
        }))
        .source(source)
        .build()
}

/// Create a project updated event
pub fn project_updated(project_id: &str, changes: serde_json::Value, source: &str) -> Event {
    EventBuilder::new("project.updated")
        .data(json!({
            "project_id": project_id,
            "changes": changes
        }))
        .source(source)
        .build()
}

/// Create an account updated event
pub fn account_updated(account_id: &str, pubkey: &str, source: &str) -> Event {
    EventBuilder::new("account.updated")
        .data(json!({
            "account_id": account_id,
            "pubkey": pubkey
        }))
        .source(source)
        .build()
}

/// Create a transaction sent event
pub fn transaction_sent(signature: &str, source: &str) -> Event {
    EventBuilder::new("transaction.sent")
        .data(json!({
            "signature": signature
        }))
        .source(source)
        .build()
}

/// Create a Solana connected event
pub fn solana_connected(network: &str, source: &str) -> Event {
    EventBuilder::new("solana.connected")
        .data(json!({
            "network": network
        }))
        .source(source)
        .build()
}

/// Create a notification event
pub fn notification(notification_type: &str, title: &str, message: &str, source: &str) -> Event {
    EventBuilder::new("ui.notification")
        .data(json!({
            "type": notification_type,
            "title": title,
            "message": message
        }))
        .source(source)
        .build()
}

/// Create an error event
pub fn error(error_message: &str, error_type: &str, source: &str) -> Event {
    EventBuilder::new("error.occurred")
        .data(json!({
            "error_message": error_message,
            "error_type": error_type
        }))
        .source(source)
        .build()
}
