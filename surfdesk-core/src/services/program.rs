//! Program Service for managing Solana programs and program deployment
//!
//! This service provides functionality for creating, building, deploying, and managing
//! Solana programs. It handles program compilation, deployment to the network,
//! and interaction with deployed programs through the custom RPC service.

use crate::services::{AsyncService, Configurable, Service, ServiceError, ServiceResult};
use crate::services::solana_rpc::{SolanaRpcService, validate_address};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration for program service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramServiceConfig {
    /// Default program deployment path
    pub deployment_path: PathBuf,
    /// Whether to optimize programs before deployment
    pub optimize_on_deploy: bool,
    /// Maximum program size in bytes
    pub max_program_size: usize,
    /// Default program account rent exemption in lamports
    pub default_rent_exemption: u64,
}

impl Default for ProgramServiceConfig {
    fn default() -> Self {
        Self {
            deployment_path: dirs::home_dir()
                .unwrap_or_default()
                .join(".surfdesk")
                .join("programs"),
            optimize_on_deploy: true,
            max_program_size: 128 * 1024, // 128KB default limit
            default_rent_exemption: 1_000_000_000, // 1 SOL
        }
    }
}

/// Program information and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    /// Program ID (address)
    pub program_id: String,
    /// Program name
    pub name: String,
    /// Program description
    pub description: String,
    /// Program version
    pub version: String,
    /// Program binary data
    pub binary_data: Vec<u8>,
    /// Program source code (if available)
    pub source_code: Option<String>,
    /// Deployment information
    pub deployment: Option<ProgramDeployment>,
    /// Program metadata
    pub metadata: ProgramMetadata,
    /// Creation timestamp
    pub created_at: String,
    /// Last updated timestamp
    pub updated_at: String,
}

/// Program deployment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramDeployment {
    /// Deployment transaction signature
    pub signature: String,
    /// Deployment slot
    pub slot: u64,
    /// Deployment status
    pub status: DeploymentStatus,
    /// Deployment timestamp
    pub deployed_at: String,
    /// Network where deployed
    pub network: String,
}

/// Deployment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeploymentStatus {
    Pending,
    Confirmed,
    Failed,
    Unknown,
}

/// Program metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramMetadata {
    /// Program author
    pub author: Option<String>,
    /// Program license
    pub license: Option<String>,
    /// Program tags
    pub tags: Vec<String>,
    /// Program category
    pub category: ProgramCategory,
    /// Whether the program is public
    pub is_public: bool,
    /// Program dependencies
    pub dependencies: Vec<String>,
}

/// Program categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProgramCategory {
    DeFi,
    NFT,
    Gaming,
    Social,
    Infrastructure,
    Utility,
    Other,
}

/// Program template for no-code creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramTemplate {
    /// Template ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template category
    pub category: ProgramCategory,
    /// Template components
    pub components: Vec<TemplateComponent>,
    /// Default configuration
    pub default_config: HashMap<String, serde_json::Value>,
}

/// Template component for visual program building
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateComponent {
    /// Component ID
    pub id: String,
    /// Component name
    pub name: String,
    /// Component type
    pub component_type: ComponentType,
    /// Component configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Position in canvas
    pub position: (i32, i32),
    /// Connections to other components
    pub connections: Vec<String>,
}

/// Component types for visual programming
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ComponentType {
    /// Account management component
    Account,
    /// Data storage component
    DataStorage,
    /// Validation component
    Validation,
    /// Math operation component
    Math,
    /// Conditional logic component
    Conditional,
    /// Loop component
    Loop,
    /// Function call component
    Function,
    /// Event handler component
    Event,
    /// Custom component
    Custom(String),
}

/// Service for managing Solana programs
pub struct ProgramService {
    config: ProgramServiceConfig,
    programs: HashMap<String, Program>,
    templates: HashMap<String, ProgramTemplate>,
    rpc_service: SolanaRpcService,
}

impl ProgramService {
    /// Create a new program service with default configuration
    pub fn new() -> Self {
        Self::with_config(ProgramServiceConfig::default())
    }

    /// Create a new program service with custom configuration
    pub fn with_config(config: ProgramServiceConfig) -> Self {
        let mut service = Self {
            rpc_service: SolanaRpcService::new(),
            programs: HashMap::new(),
            templates: HashMap::new(),
            config,
        };

        // Initialize default templates
        service.initialize_default_templates();
        service
    }

    /// Create a new program from template
    pub async fn create_program_from_template(
        &mut self,
        template_id: &str,
        name: String,
        description: String,
        config_overrides: Option<HashMap<String, serde_json::Value>>,
    ) -> ServiceResult<Program> {
        let template = self.templates.get(template_id)
            .ok_or_else(|| ServiceError::Validation(format!("Template not found: {}", template_id)))?;

        let program_id = self.generate_program_id(&name);
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Generate program code from template
        let source_code = self.generate_code_from_template(template, config_overrides.as_ref())?;

        // Compile source code to binary (mock implementation)
        let binary_data = self.compile_source_code(&source_code)?;

        let program = Program {
            program_id: program_id.clone(),
            name,
            description,
            version: "1.0.0".to_string(),
            binary_data,
            source_code: Some(source_code),
            deployment: None,
            metadata: ProgramMetadata {
                author: None,
                license: Some("MIT".to_string()),
                tags: vec![template.category.to_string().to_lowercase()],
                category: template.category.clone(),
                is_public: false,
                dependencies: Vec::new(),
            },
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        self.programs.insert(program_id.clone(), program.clone());
        self.save_programs().await?;

        Ok(program)
    }

    /// Create a custom program from components
    pub async fn create_program_from_components(
        &mut self,
        name: String,
        description: String,
        components: Vec<TemplateComponent>,
    ) -> ServiceResult<Program> {
        let program_id = self.generate_program_id(&name);
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Generate program code from components
        let source_code = self.generate_code_from_components(&components)?;

        // Compile source code to binary
        let binary_data = self.compile_source_code(&source_code)?;

        let program = Program {
            program_id: program_id.clone(),
            name,
            description,
            version: "1.0.0".to_string(),
            binary_data,
            source_code: Some(source_code),
            deployment: None,
            metadata: ProgramMetadata {
                author: None,
                license: Some("MIT".to_string()),
                tags: vec!["custom".to_string()],
                category: ProgramCategory::Utility,
                is_public: false,
                dependencies: Vec::new(),
            },
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        self.programs.insert(program_id.clone(), program.clone());
        self.save_programs().await?;

        Ok(program)
    }

    /// Deploy a program to the network
    pub async fn deploy_program(&mut self, program_id: &str) -> ServiceResult<ProgramDeployment> {
        let program = self.programs.get_mut(program_id)
            .ok_or_else(|| ServiceError::Validation(format!("Program not found: {}", program_id)))?;

        if program.binary_data.len() > self.config.max_program_size {
            return Err(ServiceError::Validation(
                format!("Program size exceeds maximum limit: {} > {} bytes",
                       program.binary_data.len(), self.config.max_program_size)
            ));
        }

        // Deploy program via RPC service
        let deployment_info = self.rpc_service.deploy_program(&program.binary_data).await?;

        let deployment = ProgramDeployment {
            signature: deployment_info.signature.clone(),
            slot: deployment_info.slot,
            status: DeploymentStatus::Confirmed,
            deployed_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            network: "local".to_string(), // Would be determined from RPC config
        };

        program.deployment = Some(deployment.clone());
        program.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        self.save_programs().await?;

        Ok(deployment)
    }

    /// Get program by ID
    pub fn get_program(&self, program_id: &str) -> Option<&Program> {
        self.programs.get(program_id)
    }

    /// Get all programs
    pub fn get_all_programs(&self) -> Vec<&Program> {
        self.programs.values().collect()
    }

    /// Get programs by category
    pub fn get_programs_by_category(&self, category: ProgramCategory) -> Vec<&Program> {
        self.programs.values()
            .filter(|p| p.metadata.category == category)
            .collect()
    }

    /// Update program
    pub async fn update_program(&mut self, program_id: &str, name: Option<String>, description: Option<String>) -> ServiceResult<()> {
        let program = self.programs.get_mut(program_id)
            .ok_or_else(|| ServiceError::Validation(format!("Program not found: {}", program_id)))?;

        if let Some(new_name) = name {
            program.name = new_name;
        }

        if let Some(new_description) = description {
            program.description = new_description;
        }

        program.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.save_programs().await?;

        Ok(())
    }

    /// Delete a program
    pub async fn delete_program(&mut self, program_id: &str) -> ServiceResult<()> {
        if self.programs.remove(program_id).is_none() {
            return Err(ServiceError::Validation(format!("Program not found: {}", program_id)));
        }

        self.save_programs().await?;
        Ok(())
    }

    /// Get available templates
    pub fn get_templates(&self) -> Vec<&ProgramTemplate> {
        self.templates.values().collect()
    }

    /// Get template by ID
    pub fn get_template(&self, template_id: &str) -> Option<&ProgramTemplate> {
        self.templates.get(template_id)
    }

    /// Test a deployed program
    pub async fn test_program(&self, program_id: &str, test_data: HashMap<String, serde_json::Value>) -> ServiceResult<TestResult> {
        let program = self.programs.get(program_id)
            .ok_or_else(|| ServiceError::Validation(format!("Program not found: {}", program_id)))?;

        if program.deployment.is_none() {
            return Err(ServiceError::Validation("Program is not deployed".to_string()));
        }

        // In a real implementation, this would call the deployed program with test data
        // For now, return a mock test result
        Ok(TestResult {
            program_id: program_id.to_string(),
            test_name: "Basic functionality test".to_string(),
            status: TestStatus::Passed,
            execution_time_ms: 150,
            result_data: Some(json!({
                "output": "Test successful",
                "gas_used": 50000
            })),
            error_message: None,
            timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    /// Initialize default program templates
    fn initialize_default_templates(&mut self) {
        // Hello World template
        let hello_world = ProgramTemplate {
            id: "hello_world".to_string(),
            name: "Hello World".to_string(),
            description: "Simple program that returns a greeting".to_string(),
            category: ProgramCategory::Utility,
            components: vec![
                TemplateComponent {
                    id: "greeting".to_string(),
                    name: "Greeting Message".to_string(),
                    component_type: ComponentType::DataStorage,
                    config: {
                        let mut config = HashMap::new();
                        config.insert("message".to_string(), json!("Hello, World!"));
                        config
                    },
                    position: (0, 0),
                    connections: Vec::new(),
                }
            ],
            default_config: HashMap::new(),
        };

        // Counter template
        let counter = ProgramTemplate {
            id: "counter".to_string(),
            name: "Counter".to_string(),
            description: "Program that maintains a counter".to_string(),
            category: ProgramCategory::Utility,
            components: vec![
                TemplateComponent {
                    id: "init".to_string(),
                    name: "Initialize Counter".to_string(),
                    component_type: ComponentType::Account,
                    config: {
                        let mut config = HashMap::new();
                        config.insert("initial_value".to_string(), json!(0));
                        config
                    },
                    position: (0, 0),
                    connections: vec!["increment".to_string()],
                },
                TemplateComponent {
                    id: "increment".to_string(),
                    name: "Increment Counter".to_string(),
                    component_type: ComponentType::Math,
                    config: {
                        let mut config = HashMap::new();
                        config.insert("operation".to_string(), json!("add"));
                        config.insert("value".to_string(), json!(1));
                        config
                    },
                    position: (100, 0),
                    connections: Vec::new(),
                }
            ],
            default_config: HashMap::new(),
        };

        self.templates.insert(hello_world.id.clone(), hello_world);
        self.templates.insert(counter.id.clone(), counter);
    }

    /// Generate code from template
    fn generate_code_from_template(&self, template: &ProgramTemplate, config_overrides: Option<&HashMap<String, serde_json::Value>>) -> ServiceResult<String> {
        let mut code = format!(
            r#"// Generated program from template: {}
// Category: {}
// Components: {}

use solana_program::{{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
}};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {{
"#,
            template.name,
            template.category,
            template.components.len()
        );

        // Add component-specific code
        for component in &template.components {
            match component.component_type {
                ComponentType::DataStorage => {
                    code.push_str(r#"
    // Data storage component
    let message = "Hello, World!";
    msg!("Message: {}", message);
"#);
                }
                ComponentType::Account => {
                    code.push_str(r#"
    // Account initialization component
    // TODO: Initialize account data structure
    msg!("Account initialized");
"#);
                }
                ComponentType::Math => {
                    code.push_str(r#"
    // Math operation component
    // TODO: Perform mathematical operation
    msg!("Math operation performed");
"#);
                }
                _ => {
                    code.push_str(&format!(
                        r#"
    // {:?} component
    msg!("{:?} component executed");
"#,
                        component.component_type,
                        component.component_type
                    ));
                }
            }
        }

        code.push_str(r#"

    Ok(())
}
"#);

        Ok(code)
    }

    /// Generate code from components
    fn generate_code_from_components(&self, components: &[TemplateComponent]) -> ServiceResult<String> {
        let mut code = String::from(
            r#"// Generated program from custom components

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
"#
        );

        for component in components {
            code.push_str(&format!(
                r#"
    // Component: {} ({:?})
    msg!("Executing component: {}");
"#,
                component.name,
                component.component_type,
                component.name
            ));
        }

        code.push_str(r#"

    Ok(())
}
"#);

        Ok(code)
    }

    /// Compile source code to binary (mock implementation)
    fn compile_source_code(&self, source_code: &str) -> ServiceResult<Vec<u8>> {
        // In a real implementation, this would invoke the Rust compiler or Solana toolchain
        // For now, return the source code as bytes
        Ok(source_code.as_bytes().to_vec())
    }

    /// Generate a program ID from program name
    fn generate_program_id(&self, name: &str) -> String {
        use sha2::{Sha256, Digest};
        use bs58;

        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        hasher.update(chrono::Utc::now().timestamp_nanos().to_be_bytes());
        let hash = hasher.finalize();

        bs58::encode(&hash[..32]).into_string()
    }

    /// Save programs to storage
    async fn save_programs(&self) -> ServiceResult<()> {
        // In a real implementation, this would save programs to disk
        tracing::debug!("Saving {} programs to storage", self.programs.len());
        Ok(())
    }

    /// Load programs from storage
    async fn load_programs(&mut self) -> ServiceResult<()> {
        // In a real implementation, this would load programs from disk
        tracing::debug!("Loading programs from storage");
        Ok(())
    }
}

impl Service for ProgramService {
    fn initialize(&mut self) -> ServiceResult<()> {
        tracing::info!("Program service initialized with {} templates", self.templates.len());
        Ok(())
    }

    fn health_check(&self) -> ServiceResult<bool> {
        Ok(!self.templates.is_empty())
    }

    fn shutdown(&mut self) -> ServiceResult<()> {
        let _ = self.save_programs();
        tracing::info!("Program service shutdown");
        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncService for ProgramService {
    async fn initialize_async(&mut self) -> ServiceResult<()> {
        self.initialize()?;
        self.load_programs().await?;
        Ok(())
    }

    async fn health_check_async(&self) -> ServiceResult<bool> {
        self.rpc_service.health_check_async().await
    }

    async fn shutdown_async(&mut self) -> ServiceResult<()> {
        self.save_programs().await?;
        self.shutdown()?;
        Ok(())
    }
}

impl Configurable for ProgramService {
    type Config = ProgramServiceConfig;

    fn configure(&mut self, config: Self::Config) -> ServiceResult<()> {
        self.config = config;
        tracing::info!("Program service reconfigured");
        Ok(())
    }

    fn get_config(&self) -> &Self::Config {
        &self.config
    }
}

/// Test result for program testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    /// Program ID tested
    pub program_id: String,
    /// Test name
    pub test_name: String,
    /// Test status
    pub status: TestStatus,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Test result data
    pub result_data: Option<serde_json::Value>,
    /// Error message if test failed
    pub error_message: Option<String>,
    /// Test timestamp
    pub timestamp: String,
}

/// Test status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TestStatus {
    Passed,
    Failed,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_service_creation() {
        let service = ProgramService::new();
        assert!(!service.get_templates().is_empty());
        assert_eq!(service.get_all_programs().len(), 0);
    }

    #[test]
    fn test_template_generation() {
        let service = ProgramService::new();
        let templates = service.get_templates();
        assert!(!templates.is_empty());

        let hello_world = service.get_template("hello_world");
        assert!(hello_world.is_some());
        assert_eq!(hello_world.unwrap().name, "Hello World");
    }

    #[test]
    fn test_component_types() {
        assert_eq!(ComponentType::Account, ComponentType::Account);
        assert_ne!(ComponentType::Account, ComponentType::DataStorage);
    }

    #[test]
    fn test_program_categories() {
        assert_eq!(ProgramCategory::DeFi, ProgramCategory::DeFi);
        assert_ne!(ProgramCategory::DeFi, ProgramCategory::NFT);
    }

    #[test]
    fn test_deployment_status() {
        assert_eq!(DeploymentStatus::Pending, DeploymentStatus::Pending);
        assert_ne!(DeploymentStatus::Pending, DeploymentStatus::Confirmed);
    }

    #[test]
    fn test_program_id_generation() {
        let service = ProgramService::new();
        let id1 = service.generate_program_id("test");
        let id2 = service.generate_program_id("test");

        // IDs should be different due to timestamp
        assert_ne!(id1, id2);
        assert!(validate_address(&id1));
        assert!(validate_address(&id2));
    }

    #[test]
    fn test_code_generation() {
        let service = ProgramService::new();
        let template = service.get_template("hello_world").unwrap();

        let code = service.generate_code_from_template(template, None).unwrap();
        assert!(code.contains("process_instruction"));
        assert!(code.contains("Hello, World!"));
    }
}
