# AI Agent Working Guidelines

## Purpose

This document defines the rules, processes, and safety guidelines for AI agents working on the SurfDesk project. It ensures consistent, high-quality contributions while maintaining code integrity and project standards.

## Pre-Work Requirements (Before Starting)

### 1. Context Understanding
- **Mandatory**: Read relevant `(lib)_CONTEXT.md` files for modules being modified
- **Required**: Review current implementation and existing tests
- **Recommended**: Check recent commits and pull requests for related changes

### 2. Environment Validation
- Verify tool availability and versions
- Check project dependencies are current
- Ensure test environment is functional
- Validate development setup (surfnet, Rust toolchain)

### 3. Impact Assessment
- Identify all files and modules that could be affected
- Consider backward compatibility implications
- Evaluate performance impact of proposed changes
- Check for potential security implications

### 4. Planning Checklist
- [ ] Clear understanding of requirements and acceptance criteria
- [ ] Implementation approach defined and documented
- [ ] Test cases identified and planned
- [ ] Rollback strategy considered
- [ ] Documentation updates planned

## During-Work Guidelines

### 1. Code Quality Standards
- Follow Rust best practices and idiomatic patterns
- Maintain consistent formatting with existing codebase
- Write clear, descriptive variable and function names
- Add appropriate comments for complex logic
- Handle errors gracefully and comprehensively

### 2. Testing Requirements
- Write unit tests for all new functions
- Add integration tests for module interactions
- Include edge cases and error conditions
- Maintain test coverage above 80%
- Use property-based testing where applicable

### 3. Documentation Updates
- Update relevant `(lib)_CONTEXT.md` files
- Add or modify code comments
- Update API documentation
- Include usage examples in docstrings
- Document any breaking changes

### 4. Safety Checks
- Never commit or push without review
- Validate all inputs and handle malicious data
- Use secure coding practices
- Avoid hardcoding credentials or sensitive data
- Follow principle of least privilege

## Post-Work Requirements (Before Completing)

### 1. Verification Checklist
- [ ] All tests pass locally
- [ ] Code compiles without warnings
- [ ] Lint checks pass (clippy, rustfmt)
- [ ] Documentation builds successfully
- [ ] No security vulnerabilities detected
- [ ] Performance benchmarks meet expectations

### 2. Integration Testing
- Test with other project modules
- Verify end-to-end functionality
- Test with different surfnet configurations
- Validate error handling in integration scenarios
- Check resource usage and memory leaks

### 3. Documentation Review
- Ensure all documentation is accurate and current
- Verify examples actually work
- Check cross-references are valid
- Update changelog if necessary
- Review API documentation for completeness

### 4. Final Validation
- Perform a final review of all changes
- Check for unintended side effects
- Verify the solution meets original requirements
- Ensure no debugging code or test artifacts remain
- Confirm project builds and runs successfully

## Emergency Procedures

### 1. Rollback Protocol
If critical issues are discovered:
- Immediately stop further work
- Revert to last known good state
- Document the issue and root cause
- Notify project maintainers
- Plan safer approach before retrying

### 2. Security Incident Response
If security vulnerabilities are found:
- Stop all work on affected areas
- Document potential impact
- Follow responsible disclosure practices
- Work with security team for mitigation
- Update procedures to prevent recurrence

## Continuous Improvement

### 1. Process Feedback
- Document lessons learned from each task
- Suggest improvements to these guidelines
- Share successful patterns and approaches
- Identify areas needing better tooling

### 2. Knowledge Sharing
- Update context files with new insights
- Share debugging techniques discovered
- Document integration patterns
- Contribute to project FAQ or troubleshooting guide

## AI-Specific Considerations

### 1. Limitations Awareness
- Recognize when human expertise is required
- Ask for clarification on ambiguous requirements
- Escalate complex architectural decisions
- Validate assumptions with project maintainers

### 2. Learning and Adaptation
- Learn from code review feedback
- Adapt to project-specific patterns
- Improve understanding of domain knowledge
- Refine approaches based on results

### 3. Communication Standards
- Be transparent about uncertainty
- Explain reasoning for major decisions
- Provide clear status updates
- Ask questions when requirements are unclear

## Enforcement and Compliance

### 1. Automated Checks
- CI/CD pipeline enforces testing requirements
- Automated documentation validation
- Code quality gates (clippy, rustfmt)
- Security scanning integration

### 2. Manual Review
- Code review by human maintainers
- Architecture review for significant changes
- Documentation review for accuracy
- User experience review for UI changes

### 3. Accountability
- Track adherence to guidelines
- Document exceptions and justifications
- Regular review of guideline effectiveness
- Updates based on project evolution

---

**Note**: These guidelines are living documents and will evolve as the project grows. All contributors should review them regularly and suggest improvements.