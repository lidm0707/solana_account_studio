# Surfdesk AI Agent Guidelines

## Overview
This document provides comprehensive guidelines for the AI assistant working on the Surfdesk project. It defines the development philosophy, constraints, and workflow for continuous improvement cycles.

## Project Context

### Core Mission
Surfdesk is a no-code web application for Solana blockchain development that enables users to create, deploy, and manage programs without deep programming knowledge.

### Technical Stack
- **Frontend**: Dioxus 0.6.3 (Rust-based web framework)
- **Backend**: Custom solana_rpc implementation (planned)
- **Local Network**: Surfpool (mainnet simulation on port 8999) (planned)
- **Architecture**: Domain-Driven Design (DDD)
- **Encoding**: Base58 for Solana addresses
- **Styling**: Native Dioxus styling (no external CSS frameworks)
- **Platform**: Web (previously desktop, now web-based)

### Key Constraints
- **No External SDKs**: Must use custom solana_rpc (no Anchor or Solana SDK)
- **Dioxus Limitations**: Cannot use Arc in Rust code
- **Memory Safety**: Proper signal-based state management
- **Zero Tolerance**: No errors or warnings allowed
- **Web Platform**: Application runs on web platform (http://127.0.0.1:8080)
- **Clean Architecture**: Recent project cleanup removed unused files and streamlined structure

## Development Workflow

### The 3-Step Cycle
Each development cycle must follow this exact sequence:

#### Step 1: Documentation Analysis
- Read all project documentation
- Collect all errors, warnings, and TODO items
- Analyze project structure and requirements
- Summarize findings and plan fixes
- Verify development server functionality (`dx serve`)
- Analyze recent structural changes and removed files

#### Step 2: Error Resolution
- Fix all errors first (highest priority)
- Resolve warnings and improve code quality
- Ensure no new issues are introduced
- Validate all fixes work correctly

#### Step 3: Documentation Update & Git Push
- Update agent.md with new insights
- Review and improve ROADMAP.md
- Update README.md if needed
- Commit and push changes
- Ensure development server runs correctly
- Validate all documentation reflects current project state

### Quality Standards
- **Build Success**: Must compile without errors
- **Zero Warnings**: All warnings must be resolved
- **Documentation**: Always update docs after changes
- **Testing**: Ensure functionality works correctly
- **Git Integration**: Every cycle must end with git push

## Core Features Implementation

### 1. Surfpool Integration
Surfpool is a 3rd party program that:
- Installs locally first time
- Forks mainnet to simulation network on port 8999
- Allows interaction with simulated network
- Must have start/kill functionality

**Implementation Requirements:**
- Process lifecycle management
- Network status monitoring
- Error handling and recovery
- Configuration management

### 2. No-Code Program Builder
Visual interface for creating Solana programs:
- Drag-and-drop program creation
- Component-based architecture
- Code generation for solana_rpc
- Template system for common patterns

**Implementation Requirements:**
- Intuitive visual editor
- Real-time validation
- Code preview functionality
- Export/import capabilities

### 3. Custom RPC Implementation
Build solana_rpc without external dependencies:
- HTTP client for JSON-RPC
- Base58 encoding/decoding
- Transaction building and signing
- Account management
- Program deployment

**Implementation Requirements:**
- Clean API design
- Comprehensive error handling
- Efficient data structures
- Security considerations

### 4. Account Management
Wallet and account functionality:
- Wallet creation and management
- Balance tracking
- Transaction history
- Multi-account support
- Security features

**Implementation Requirements:**
- Secure key storage
- Mnemonic support
- Backup/restore functionality
- User-friendly interface

## Architecture Guidelines

### Domain-Driven Design (DDD)
Implement clear domain boundaries:
- **Domain Layer**: Business logic and rules
- **Application Layer**: Use cases and workflows
- **Infrastructure Layer**: External integrations
- **Interface Layer**: User interface components

### State Management with Dioxus
- Use signals for reactive state
- Avoid Arc (Dioxus constraint)
- Implement proper state isolation
- Use memoization for performance

### Error Handling Philosophy
- Comprehensive error coverage
- User-friendly error messages
- Graceful degradation
- Recovery mechanisms
- Logging and monitoring

## Code Quality Standards

### Rust/Dioxus Specific
- Follow Rust best practices
- Use Dioxus patterns correctly
- Implement proper component lifecycle
- Use signals for state management
- Avoid unwrap() without proper error handling

### Security Requirements
- Secure key storage
- Input validation
- Safe network communication
- Memory safety
- Audit logging

### Performance Considerations
- Efficient memory usage
- Fast startup times
- Responsive UI
- Minimal network calls
- Caching strategies

## Testing Strategy

### Unit Testing
- Test all business logic
- Mock external dependencies
- Cover edge cases
- Performance benchmarks

### Integration Testing
- Test component interactions
- Validate RPC calls
- Test network operations
- User workflow testing

### Manual Testing
- Feature validation
- User experience testing
- Error scenario testing
- Performance validation

## Documentation Standards

### Code Documentation
- Comprehensive inline comments
- Function and struct documentation
- Usage examples
- Architecture decisions

### User Documentation
- Clear installation instructions
- Feature tutorials
- Troubleshooting guides
- FAQ section

### Technical Documentation
- API documentation
- Architecture diagrams
- Development setup
- Contribution guidelines

## Git Workflow

### Branch Strategy
- Main branch for production
- Feature branches for development
- Regular integration and testing
- Clean commit history

### Commit Standards
- Descriptive commit messages
- Atomic commits
- Proper attribution
- Link to issues/tickets

### Release Process
- Version tagging
- Release notes
- Changelog updates
- Deployment validation

## Troubleshooting Guidelines

### Common Issues
- Build failures: Check dependencies and syntax
- Runtime errors: Review logs and error messages
- Performance issues: Profile and optimize
- Network problems: Verify connectivity and configuration

### Debugging Approach
- Use systematic debugging
- Check logs first
- Reproduce issues consistently
- Document findings and solutions

## Continuous Improvement

### Learning Objectives
- Understand Dioxus patterns deeply
- Master Solana protocol concepts
- Improve Rust programming skills
- Learn user experience design

### Process Refinement
- Regular workflow evaluation
- Tool improvement suggestions
- Documentation updates
- Best practice evolution

## Success Metrics

### Technical Metrics
- Zero build errors
- Zero warnings
- Test coverage >90%
- Performance benchmarks
- Security audit results

### User Metrics
- Feature completion rate
- User satisfaction scores
- Bug report frequency
- Feature adoption rates
- Support ticket volume

---

## Emergency Procedures

### Critical Issues
1. Stop current development
2. Identify root cause
3. Implement immediate fix
4. Test thoroughly
5. Deploy with monitoring
6. Document lessons learned

### Rollback Procedures
1. Identify last stable version
2. Prepare rollback plan
3. Execute rollback
4. Validate functionality
5. Communicate status
6. Plan forward approach

---

**Last Updated**: 2025-10-19 (Project Restructure Phase)  
**Next Review**: After each development cycle  
**Maintainer**: AI Development Assistant

Remember: Every cycle must end with zero errors, zero warnings, updated documentation, and a git push.

## Recent Changes (2025-10-19)

### Project Restructure
- ✅ Removed desktop version components
- ✅ Cleaned up unused documentation files
- ✅ Streamlined project structure for web platform
- ✅ Consolidated configuration files
- ✅ Removed test modules for cleaner base
- ✅ Development server working correctly on web platform

### Current State
- Web application functional at http://127.0.0.1:8080
- Clean compilation with 0 errors, 0 warnings
- Dioxus 0.6.3 web platform configuration
- Native styling implementation complete
- Core routing system working