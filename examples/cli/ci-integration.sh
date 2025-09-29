#!/bin/bash
# Example: CI/CD Integration
# Purpose: Demonstrates how to integrate Smart Code Diff into CI/CD pipelines
# Usage: ./ci-integration.sh

set -e

echo "Smart Code Diff Example: CI/CD Integration"
echo "=========================================="

# Configuration
REPO_DIR="$(dirname "$0")/mock-repo"
REPORTS_DIR="$(dirname "$0")/ci-reports"
THRESHOLD=0.7
MAX_CHANGES=10

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    if ! command -v smart-diff-cli &> /dev/null; then
        log_error "smart-diff-cli not found in PATH"
        exit 1
    fi
    
    if ! command -v jq &> /dev/null; then
        log_warn "jq not found - JSON processing will be limited"
    fi
    
    log_info "Prerequisites check completed"
}

# Setup mock repository structure
setup_mock_repo() {
    log_info "Setting up mock repository..."
    
    mkdir -p "$REPO_DIR"/{src,test,docs}
    mkdir -p "$REPORTS_DIR"
    
    # Create "before" version (main branch)
    cat > "$REPO_DIR/src/UserService.java" << 'EOF'
public class UserService {
    private UserRepository repository;
    
    public UserService(UserRepository repository) {
        this.repository = repository;
    }
    
    public User findUser(String id) {
        if (id == null || id.isEmpty()) {
            throw new IllegalArgumentException("User ID cannot be null or empty");
        }
        return repository.findById(id);
    }
    
    public void saveUser(User user) {
        if (user == null) {
            throw new IllegalArgumentException("User cannot be null");
        }
        repository.save(user);
    }
    
    public void deleteUser(String id) {
        repository.deleteById(id);
    }
}
EOF

    # Create "after" version (feature branch)
    cat > "$REPO_DIR/src/UserService_new.java" << 'EOF'
public class UserService {
    private UserRepository repository;
    private UserValidator validator;
    
    public UserService(UserRepository repository, UserValidator validator) {
        this.repository = repository;
        this.validator = validator;
    }
    
    public User findUser(String id) {
        validateUserId(id);
        return repository.findById(id);
    }
    
    public void saveUser(User user) {
        validator.validate(user);
        repository.save(user);
    }
    
    public void deleteUser(String id) {
        validateUserId(id);
        User user = findUser(id);
        if (user != null) {
            repository.deleteById(id);
        }
    }
    
    // Extracted validation method
    private void validateUserId(String id) {
        if (id == null || id.isEmpty()) {
            throw new IllegalArgumentException("User ID cannot be null or empty");
        }
    }
    
    // New method
    public List<User> findActiveUsers() {
        return repository.findByStatus("ACTIVE");
    }
}
EOF

    log_info "Mock repository created at $REPO_DIR"
}

# Simulate CI/CD pipeline steps
run_ci_pipeline() {
    log_info "Starting CI/CD pipeline simulation..."
    
    # Step 1: Code Quality Gate
    log_info "Step 1: Code Quality Gate"
    run_quality_gate
    
    # Step 2: Change Impact Analysis
    log_info "Step 2: Change Impact Analysis"
    run_impact_analysis
    
    # Step 3: Generate Reports
    log_info "Step 3: Generate Reports"
    generate_reports
    
    # Step 4: Decision Making
    log_info "Step 4: Pipeline Decision"
    make_pipeline_decision
}

# Quality gate check
run_quality_gate() {
    local result_file="$REPORTS_DIR/quality-gate.json"
    
    smart-diff-cli compare --output json \
        "$REPO_DIR/src/UserService.java" \
        "$REPO_DIR/src/UserService_new.java" > "$result_file"
    
    if command -v jq &> /dev/null; then
        local similarity=$(jq -r '.similarity' "$result_file")
        local changes=$(jq -r '.analysis.changes.total_changes' "$result_file")
        
        echo "  Similarity: $similarity"
        echo "  Total changes: $changes"
        
        # Quality gate rules
        if (( $(echo "$similarity < $THRESHOLD" | bc -l) )); then
            log_warn "Quality gate: Low similarity ($similarity < $THRESHOLD)"
            echo "QUALITY_GATE_WARNING=true" >> "$REPORTS_DIR/pipeline.env"
        else
            log_info "Quality gate: Similarity check passed ($similarity >= $THRESHOLD)"
        fi
        
        if [ "$changes" -gt "$MAX_CHANGES" ]; then
            log_warn "Quality gate: Too many changes ($changes > $MAX_CHANGES)"
            echo "QUALITY_GATE_WARNING=true" >> "$REPORTS_DIR/pipeline.env"
        else
            log_info "Quality gate: Change count acceptable ($changes <= $MAX_CHANGES)"
        fi
    else
        log_warn "jq not available - skipping detailed quality gate analysis"
    fi
}

# Impact analysis
run_impact_analysis() {
    local impact_file="$REPORTS_DIR/impact-analysis.txt"
    
    smart-diff-cli compare \
        "$REPO_DIR/src/UserService.java" \
        "$REPO_DIR/src/UserService_new.java" > "$impact_file"
    
    # Extract key metrics
    local refactoring_patterns=$(grep -c "Refactoring Pattern" "$impact_file" || echo "0")
    local functions_added=$(grep -c "added" "$impact_file" || echo "0")
    local functions_modified=$(grep -c "modified\|renamed" "$impact_file" || echo "0")
    
    echo "  Refactoring patterns detected: $refactoring_patterns"
    echo "  Functions added: $functions_added"
    echo "  Functions modified: $functions_modified"
    
    # Store metrics for later use
    cat > "$REPORTS_DIR/metrics.env" << EOF
REFACTORING_PATTERNS=$refactoring_patterns
FUNCTIONS_ADDED=$functions_added
FUNCTIONS_MODIFIED=$functions_modified
EOF
}

# Generate comprehensive reports
generate_reports() {
    # HTML report for human review
    smart-diff-cli compare --output html \
        "$REPO_DIR/src/UserService.java" \
        "$REPO_DIR/src/UserService_new.java" > "$REPORTS_DIR/change-report.html"
    
    # XML report for tools integration
    smart-diff-cli compare --output xml \
        "$REPO_DIR/src/UserService.java" \
        "$REPO_DIR/src/UserService_new.java" > "$REPORTS_DIR/change-report.xml"
    
    # Summary report
    cat > "$REPORTS_DIR/summary.md" << 'EOF'
# Change Analysis Summary

## Overview
This report summarizes the changes detected between the main branch and the feature branch.

## Files Analyzed
- `src/UserService.java` (before)
- `src/UserService_new.java` (after)

## Key Findings
- Method extraction refactoring detected
- New dependency injection parameter added
- Enhanced error handling implemented
- New functionality added (findActiveUsers)

## Recommendations
- Review the extracted validation method for consistency
- Ensure proper testing of new dependency injection
- Validate error handling improvements
- Test new findActiveUsers functionality

## Next Steps
- Manual code review recommended
- Update unit tests for new functionality
- Update documentation for API changes
EOF

    log_info "Reports generated in $REPORTS_DIR/"
    echo "  - change-report.html (visual review)"
    echo "  - change-report.xml (tools integration)"
    echo "  - quality-gate.json (metrics)"
    echo "  - impact-analysis.txt (detailed analysis)"
    echo "  - summary.md (executive summary)"
}

# Make pipeline decision
make_pipeline_decision() {
    local decision="PROCEED"
    local warnings=0
    
    # Check for quality gate warnings
    if [ -f "$REPORTS_DIR/pipeline.env" ]; then
        source "$REPORTS_DIR/pipeline.env"
        if [ "$QUALITY_GATE_WARNING" = "true" ]; then
            warnings=$((warnings + 1))
            log_warn "Quality gate warnings detected"
        fi
    fi
    
    # Check for high-impact changes
    if [ -f "$REPORTS_DIR/metrics.env" ]; then
        source "$REPORTS_DIR/metrics.env"
        if [ "$FUNCTIONS_MODIFIED" -gt 3 ]; then
            warnings=$((warnings + 1))
            log_warn "High number of function modifications detected"
        fi
    fi
    
    # Make decision
    if [ $warnings -gt 1 ]; then
        decision="MANUAL_REVIEW_REQUIRED"
        log_warn "Pipeline decision: Manual review required"
        echo "PIPELINE_DECISION=MANUAL_REVIEW_REQUIRED" >> "$REPORTS_DIR/pipeline.env"
    elif [ $warnings -eq 1 ]; then
        decision="PROCEED_WITH_CAUTION"
        log_warn "Pipeline decision: Proceed with caution"
        echo "PIPELINE_DECISION=PROCEED_WITH_CAUTION" >> "$REPORTS_DIR/pipeline.env"
    else
        decision="PROCEED"
        log_info "Pipeline decision: Proceed with deployment"
        echo "PIPELINE_DECISION=PROCEED" >> "$REPORTS_DIR/pipeline.env"
    fi
    
    echo "FINAL_DECISION=$decision"
}

# GitHub Actions integration example
generate_github_actions_workflow() {
    local workflow_file="$REPORTS_DIR/github-workflow.yml"
    
    cat > "$workflow_file" << 'EOF'
name: Smart Code Diff Analysis

on:
  pull_request:
    branches: [ main ]

jobs:
  code-analysis:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
      with:
        fetch-depth: 0
    
    - name: Setup Smart Code Diff
      run: |
        # Install Smart Code Diff (replace with actual installation)
        curl -L https://github.com/smart-code-diff/releases/latest/download/smart-diff-cli-linux.tar.gz | tar xz
        chmod +x smart-diff-cli
        sudo mv smart-diff-cli /usr/local/bin/
    
    - name: Analyze Changes
      run: |
        # Get changed files
        git diff --name-only origin/main...HEAD | grep -E '\.(java|py|js|cpp|c)$' > changed_files.txt
        
        # Analyze each changed file
        mkdir -p reports
        while IFS= read -r file; do
          if [ -f "$file" ]; then
            echo "Analyzing $file"
            git show origin/main:"$file" > "reports/$(basename "$file").old" 2>/dev/null || echo "New file: $file"
            if [ -f "reports/$(basename "$file").old" ]; then
              smart-diff-cli compare --output json "reports/$(basename "$file").old" "$file" > "reports/$(basename "$file").json"
            fi
          fi
        done < changed_files.txt
    
    - name: Generate Summary
      run: |
        # Process results and generate summary
        echo "# Code Analysis Summary" > reports/summary.md
        echo "" >> reports/summary.md
        
        for json_file in reports/*.json; do
          if [ -f "$json_file" ]; then
            filename=$(basename "$json_file" .json)
            similarity=$(jq -r '.similarity' "$json_file")
            changes=$(jq -r '.analysis.changes.total_changes' "$json_file")
            echo "- **$filename**: Similarity $similarity, Changes $changes" >> reports/summary.md
          fi
        done
    
    - name: Comment PR
      uses: actions/github-script@v6
      with:
        script: |
          const fs = require('fs');
          const summary = fs.readFileSync('reports/summary.md', 'utf8');
          
          github.rest.issues.createComment({
            issue_number: context.issue.number,
            owner: context.repo.owner,
            repo: context.repo.repo,
            body: summary
          });
    
    - name: Upload Reports
      uses: actions/upload-artifact@v3
      with:
        name: code-analysis-reports
        path: reports/
EOF

    log_info "GitHub Actions workflow generated: $workflow_file"
}

# Jenkins pipeline example
generate_jenkins_pipeline() {
    local pipeline_file="$REPORTS_DIR/Jenkinsfile"
    
    cat > "$pipeline_file" << 'EOF'
pipeline {
    agent any
    
    environment {
        SMART_DIFF_THRESHOLD = '0.7'
        MAX_CHANGES = '10'
    }
    
    stages {
        stage('Setup') {
            steps {
                script {
                    // Install Smart Code Diff if not available
                    sh '''
                        if ! command -v smart-diff-cli &> /dev/null; then
                            echo "Installing Smart Code Diff..."
                            # Add installation commands here
                        fi
                    '''
                }
            }
        }
        
        stage('Code Analysis') {
            steps {
                script {
                    // Get changed files from Git
                    def changedFiles = sh(
                        script: "git diff --name-only HEAD~1 HEAD | grep -E '\\.(java|py|js|cpp|c)\$' || true",
                        returnStdout: true
                    ).trim().split('\n')
                    
                    // Analyze each file
                    changedFiles.each { file ->
                        if (file && fileExists(file)) {
                            echo "Analyzing ${file}"
                            sh """
                                git show HEAD~1:${file} > ${file}.old 2>/dev/null || echo "New file: ${file}"
                                if [ -f "${file}.old" ]; then
                                    smart-diff-cli compare --output json ${file}.old ${file} > ${file}.analysis.json
                                fi
                            """
                        }
                    }
                }
            }
        }
        
        stage('Quality Gate') {
            steps {
                script {
                    def qualityGatePassed = true
                    def warnings = []
                    
                    // Check analysis results
                    sh "find . -name '*.analysis.json' -exec cat {} \\; | jq -s '.' > combined-analysis.json"
                    
                    def analysisResults = readJSON file: 'combined-analysis.json'
                    
                    analysisResults.each { result ->
                        if (result.similarity < env.SMART_DIFF_THRESHOLD.toDouble()) {
                            warnings.add("Low similarity detected: ${result.similarity}")
                        }
                        if (result.analysis.changes.total_changes > env.MAX_CHANGES.toInteger()) {
                            warnings.add("Too many changes: ${result.analysis.changes.total_changes}")
                        }
                    }
                    
                    if (warnings.size() > 0) {
                        echo "Quality gate warnings:"
                        warnings.each { warning ->
                            echo "  - ${warning}"
                        }
                        
                        if (warnings.size() > 2) {
                            error("Quality gate failed: Too many warnings")
                        } else {
                            unstable("Quality gate warnings detected")
                        }
                    }
                }
            }
        }
        
        stage('Generate Reports') {
            steps {
                sh '''
                    mkdir -p reports
                    
                    # Generate HTML reports
                    find . -name "*.analysis.json" | while read json_file; do
                        base_name=$(basename "$json_file" .analysis.json)
                        if [ -f "${base_name}.old" ]; then
                            smart-diff-cli compare --output html "${base_name}.old" "$base_name" > "reports/${base_name}.html"
                        fi
                    done
                    
                    # Generate summary
                    echo "# Code Analysis Report" > reports/summary.md
                    echo "Generated on: $(date)" >> reports/summary.md
                    echo "" >> reports/summary.md
                    
                    find . -name "*.analysis.json" -exec jq -r '"- " + .analysis.files.target.path + ": Similarity " + (.similarity | tostring) + ", Changes " + (.analysis.changes.total_changes | tostring)' {} \\; >> reports/summary.md
                '''
            }
        }
    }
    
    post {
        always {
            archiveArtifacts artifacts: 'reports/**/*', fingerprint: true
            publishHTML([
                allowMissing: false,
                alwaysLinkToLastBuild: true,
                keepAll: true,
                reportDir: 'reports',
                reportFiles: '*.html',
                reportName: 'Code Analysis Report'
            ])
        }
        
        unstable {
            emailext (
                subject: "Code Analysis Warnings - ${env.JOB_NAME} #${env.BUILD_NUMBER}",
                body: "Code analysis detected potential issues. Please review the reports.",
                to: "${env.CHANGE_AUTHOR_EMAIL}"
            )
        }
        
        failure {
            emailext (
                subject: "Code Analysis Failed - ${env.JOB_NAME} #${env.BUILD_NUMBER}",
                body: "Code analysis failed quality gates. Manual review required.",
                to: "${env.CHANGE_AUTHOR_EMAIL}"
            )
        }
    }
}
EOF

    log_info "Jenkins pipeline generated: $pipeline_file"
}

# Main execution
main() {
    check_prerequisites
    setup_mock_repo
    run_ci_pipeline
    generate_github_actions_workflow
    generate_jenkins_pipeline
    
    echo
    log_info "CI/CD Integration example completed!"
    echo
    echo "Generated files:"
    echo "  - Mock repository: $REPO_DIR"
    echo "  - Analysis reports: $REPORTS_DIR"
    echo "  - GitHub Actions workflow: $REPORTS_DIR/github-workflow.yml"
    echo "  - Jenkins pipeline: $REPORTS_DIR/Jenkinsfile"
    echo
    echo "Key integration patterns demonstrated:"
    echo "1. Quality gates based on similarity thresholds"
    echo "2. Change impact analysis with automated decisions"
    echo "3. Multi-format report generation"
    echo "4. Pipeline integration with popular CI/CD tools"
    echo "5. Automated notifications and artifact archiving"
    echo
    echo "Next steps:"
    echo "- Adapt the workflows to your specific CI/CD platform"
    echo "- Customize quality gate rules for your project"
    echo "- Integrate with your notification systems"
    echo "- Set up automated report distribution"
}

# Cleanup function
cleanup() {
    if [ "$1" = "--clean" ]; then
        log_info "Cleaning up generated files..."
        rm -rf "$REPO_DIR" "$REPORTS_DIR"
        log_info "Cleanup completed"
    fi
}

# Handle cleanup argument
if [ "$1" = "--clean" ]; then
    cleanup --clean
    exit 0
fi

# Run main function
main

echo
echo "To clean up generated files, run: $0 --clean"
