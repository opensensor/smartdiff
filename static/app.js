// Smart Code Diff - Advanced Web Application
class SmartCodeDiffApp {
    constructor() {
        this.files = new Map();
        this.parsedFiles = new Map();
        this.currentView = 'explorer';
        this.apiBase = '/api';
        
        this.init();
    }

    init() {
        this.setupEventListeners();
        this.setupDragAndDrop();
        this.checkSystemHealth();
        this.loadSettings();
    }

    setupEventListeners() {
        // Navigation
        document.querySelectorAll('.nav-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const view = e.currentTarget.dataset.view;
                this.switchView(view);
            });
        });

        // File inputs
        document.getElementById('fileInput').addEventListener('change', (e) => {
            this.handleFileSelection(e.target.files, true);
        });

        document.getElementById('singleFileInput').addEventListener('change', (e) => {
            this.handleFileSelection(e.target.files, false);
        });

        // Explorer actions
        document.getElementById('parseAllBtn').addEventListener('click', () => {
            this.parseAllFiles();
        });

        document.getElementById('clearFilesBtn').addEventListener('click', () => {
            this.clearFiles();
        });

        // Compare actions
        document.getElementById('compareBtn').addEventListener('click', () => {
            this.compareFiles();
        });

        document.getElementById('loadSourceBtn').addEventListener('click', () => {
            this.loadSelectedFile('source');
        });

        document.getElementById('loadTargetBtn').addEventListener('click', () => {
            this.loadSelectedFile('target');
        });

        // Analysis actions
        document.getElementById('runAnalysisBtn').addEventListener('click', () => {
            this.runAnalysis();
        });

        // Settings actions
        document.getElementById('saveSettingsBtn').addEventListener('click', () => {
            this.saveSettings();
        });

        document.getElementById('resetSettingsBtn').addEventListener('click', () => {
            this.resetSettings();
        });

        // Health check
        document.getElementById('healthCheck').addEventListener('click', () => {
            this.checkSystemHealth();
        });

        // Threshold slider
        document.getElementById('thresholdSlider').addEventListener('input', (e) => {
            document.getElementById('thresholdValue').textContent = e.target.value;
        });
    }

    setupDragAndDrop() {
        const uploadZone = document.getElementById('uploadZone');

        uploadZone.addEventListener('dragover', (e) => {
            e.preventDefault();
            uploadZone.classList.add('dragover');
        });

        uploadZone.addEventListener('dragleave', (e) => {
            e.preventDefault();
            uploadZone.classList.remove('dragover');
        });

        uploadZone.addEventListener('drop', (e) => {
            e.preventDefault();
            uploadZone.classList.remove('dragover');
            
            const items = Array.from(e.dataTransfer.items);
            this.handleDroppedItems(items);
        });

        uploadZone.addEventListener('click', () => {
            document.getElementById('singleFileInput').click();
        });
    }

    async handleDroppedItems(items) {
        const files = [];
        
        for (const item of items) {
            if (item.kind === 'file') {
                const entry = item.webkitGetAsEntry();
                if (entry) {
                    await this.traverseFileTree(entry, files);
                }
            }
        }
        
        this.handleFileSelection(files, true);
    }

    async traverseFileTree(item, files, path = '') {
        return new Promise((resolve) => {
            if (item.isFile) {
                item.file((file) => {
                    if (this.isCodeFile(file.name)) {
                        files.push(file);
                    }
                    resolve();
                });
            } else if (item.isDirectory) {
                const dirReader = item.createReader();
                dirReader.readEntries(async (entries) => {
                    for (const entry of entries) {
                        await this.traverseFileTree(entry, files, path + item.name + '/');
                    }
                    resolve();
                });
            }
        });
    }

    isCodeFile(filename) {
        const codeExtensions = [
            '.c', '.cpp', '.cc', '.cxx', '.h', '.hpp', '.hxx',
            '.js', '.jsx', '.ts', '.tsx', '.mjs', '.cjs',
            '.py', '.pyx', '.pyi',
            '.java', '.kt', '.scala',
            '.rs', '.go', '.rb', '.php',
            '.cs', '.vb', '.fs',
            '.swift', '.m', '.mm',
            '.sh', '.bash', '.zsh',
            '.sql', '.pl', '.r',
            '.html', '.htm', '.xml',
            '.css', '.scss', '.sass', '.less',
            '.json', '.yaml', '.yml', '.toml',
            '.md', '.txt', '.cfg', '.ini'
        ];
        
        return codeExtensions.some(ext => filename.toLowerCase().endsWith(ext));
    }

    async handleFileSelection(files, isDirectory) {
        if (!files || files.length === 0) return;

        this.showLoading('Processing files...');
        
        try {
            for (const file of files) {
                if (this.isCodeFile(file.name)) {
                    const content = await this.readFileContent(file);
                    this.files.set(file.name, {
                        name: file.name,
                        path: file.webkitRelativePath || file.name,
                        content: content,
                        size: file.size,
                        lastModified: file.lastModified,
                        type: this.detectLanguage(file.name)
                    });
                }
            }
            
            this.updateFileTree();
            this.updateFileSelectors();
            this.hideLoading();
            this.showToast('Files loaded successfully', 'success');
            
        } catch (error) {
            this.hideLoading();
            this.showToast(`Error loading files: ${error.message}`, 'error');
        }
    }

    readFileContent(file) {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = (e) => resolve(e.target.result);
            reader.onerror = (e) => reject(new Error('Failed to read file'));
            reader.readAsText(file);
        });
    }

    detectLanguage(filename) {
        const ext = filename.toLowerCase().split('.').pop();
        const languageMap = {
            'c': 'C',
            'cpp': 'C++', 'cc': 'C++', 'cxx': 'C++',
            'h': 'C/C++', 'hpp': 'C++', 'hxx': 'C++',
            'js': 'JavaScript', 'jsx': 'JavaScript', 'mjs': 'JavaScript', 'cjs': 'JavaScript',
            'ts': 'TypeScript', 'tsx': 'TypeScript',
            'py': 'Python', 'pyx': 'Python', 'pyi': 'Python',
            'java': 'Java',
            'kt': 'Kotlin',
            'rs': 'Rust',
            'go': 'Go',
            'rb': 'Ruby',
            'php': 'PHP',
            'cs': 'C#',
            'swift': 'Swift',
            'sh': 'Shell', 'bash': 'Shell', 'zsh': 'Shell',
            'sql': 'SQL',
            'html': 'HTML', 'htm': 'HTML',
            'css': 'CSS', 'scss': 'SCSS', 'sass': 'SASS',
            'json': 'JSON',
            'yaml': 'YAML', 'yml': 'YAML',
            'md': 'Markdown',
            'xml': 'XML'
        };
        
        return languageMap[ext] || 'Unknown';
    }

    updateFileTree() {
        const treeContent = document.getElementById('treeContent');
        const fileTree = document.getElementById('fileTree');
        
        if (this.files.size === 0) {
            fileTree.style.display = 'none';
            return;
        }
        
        fileTree.style.display = 'block';
        treeContent.innerHTML = '';
        
        // Group files by directory
        const filesByDir = new Map();
        
        for (const [name, file] of this.files) {
            const pathParts = file.path.split('/');
            const dir = pathParts.length > 1 ? pathParts.slice(0, -1).join('/') : '';
            
            if (!filesByDir.has(dir)) {
                filesByDir.set(dir, []);
            }
            filesByDir.get(dir).push(file);
        }
        
        // Render file tree
        for (const [dir, files] of filesByDir) {
            if (dir) {
                const dirItem = this.createTreeItem('📁', dir, 'directory', `${files.length} files`);
                treeContent.appendChild(dirItem);
            }
            
            files.forEach(file => {
                const icon = this.getFileIcon(file.type);
                const info = `${file.type} • ${this.formatFileSize(file.size)}`;
                const item = this.createTreeItem(icon, file.name, 'file', info, file);
                if (dir) {
                    item.style.marginLeft = '20px';
                }
                treeContent.appendChild(item);
            });
        }
    }

    createTreeItem(icon, name, type, info, file = null) {
        const item = document.createElement('div');
        item.className = 'tree-item';
        item.dataset.type = type;
        if (file) item.dataset.filename = file.name;
        
        item.innerHTML = `
            <span class="icon">${icon}</span>
            <span class="name">${name}</span>
            <span class="info">${info}</span>
        `;
        
        if (file) {
            item.addEventListener('click', () => {
                document.querySelectorAll('.tree-item').forEach(i => i.classList.remove('selected'));
                item.classList.add('selected');
            });
        }
        
        return item;
    }

    getFileIcon(type) {
        const iconMap = {
            'C': '🔧', 'C++': '⚙️', 'C/C++': '🔧',
            'JavaScript': '🟨', 'TypeScript': '🔷',
            'Python': '🐍', 'Java': '☕', 'Kotlin': '🎯',
            'Rust': '🦀', 'Go': '🐹', 'Ruby': '💎',
            'PHP': '🐘', 'C#': '🔷', 'Swift': '🦉',
            'Shell': '🐚', 'SQL': '🗃️',
            'HTML': '🌐', 'CSS': '🎨', 'SCSS': '🎨',
            'JSON': '📋', 'YAML': '📄', 'Markdown': '📝',
            'XML': '📄'
        };
        
        return iconMap[type] || '📄';
    }

    formatFileSize(bytes) {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    }

    updateFileSelectors() {
        const sourceSelect = document.getElementById('sourceFileSelect');
        const targetSelect = document.getElementById('targetFileSelect');
        
        // Clear existing options
        sourceSelect.innerHTML = '<option value="">Select a file...</option>';
        targetSelect.innerHTML = '<option value="">Select a file...</option>';
        
        // Add file options
        for (const [name, file] of this.files) {
            const option1 = document.createElement('option');
            option1.value = name;
            option1.textContent = `${file.name} (${file.type})`;
            sourceSelect.appendChild(option1);
            
            const option2 = document.createElement('option');
            option2.value = name;
            option2.textContent = `${file.name} (${file.type})`;
            targetSelect.appendChild(option2);
        }
    }

    async parseAllFiles() {
        if (this.files.size === 0) {
            this.showToast('No files to parse', 'warning');
            return;
        }

        this.showLoading('Parsing files...');
        
        try {
            const filesArray = Array.from(this.files.values()).map(file => ({
                path: file.path,
                content: file.content
            }));

            const response = await fetch(`${this.apiBase}/analyze`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    files: filesArray,
                    options: {
                        enable_cross_file_analysis: true,
                        detect_duplicates: true,
                        calculate_complexity: true
                    }
                })
            });

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            const result = await response.json();
            this.displayParsingResults(result);
            this.hideLoading();
            this.showToast('Files parsed successfully', 'success');
            
        } catch (error) {
            this.hideLoading();
            this.showToast(`Error parsing files: ${error.message}`, 'error');
        }
    }

    displayParsingResults(result) {
        const resultsContainer = document.getElementById('parsingResults');
        const statsContainer = document.getElementById('resultsStats');
        const gridContainer = document.getElementById('resultsGrid');
        
        resultsContainer.style.display = 'block';
        
        // Update stats
        statsContainer.innerHTML = `
            <div class="stat-item">
                <span class="stat-value">${result.summary.total_files}</span>
                <span>Files</span>
            </div>
            <div class="stat-item">
                <span class="stat-value">${result.summary.total_functions}</span>
                <span>Functions</span>
            </div>
            <div class="stat-item">
                <span class="stat-value">${result.summary.average_complexity.toFixed(1)}</span>
                <span>Avg Complexity</span>
            </div>
            <div class="stat-item">
                <span class="stat-value">${(result.summary.duplicate_rate * 100).toFixed(1)}%</span>
                <span>Duplicates</span>
            </div>
        `;
        
        // Update grid
        gridContainer.innerHTML = '';
        
        result.files.forEach(fileResult => {
            const card = this.createResultCard(fileResult);
            gridContainer.appendChild(card);
        });
        
        // Store parsed results
        this.parsedFiles.set('analysis', result);
    }

    createResultCard(fileResult) {
        const card = document.createElement('div');
        card.className = 'result-card';
        
        const languageIcon = this.getFileIcon(fileResult.file.language || 'Unknown');
        
        card.innerHTML = `
            <h4>
                ${languageIcon} ${fileResult.file.path}
                <span class="language-badge">${fileResult.file.language || 'Unknown'}</span>
            </h4>
            <div class="result-metrics">
                <div class="metric">
                    <span class="metric-value">${fileResult.file.lines}</span>
                    <span class="metric-label">Lines</span>
                </div>
                <div class="metric">
                    <span class="metric-value">${fileResult.file.functions}</span>
                    <span class="metric-label">Functions</span>
                </div>
                <div class="metric">
                    <span class="metric-value">${fileResult.file.classes}</span>
                    <span class="metric-label">Classes</span>
                </div>
                <div class="metric">
                    <span class="metric-value">${fileResult.file.complexity.toFixed(1)}</span>
                    <span class="metric-label">Complexity</span>
                </div>
            </div>
            <div class="function-list">
                ${fileResult.functions.slice(0, 5).map(func => `
                    <div class="function-item">
                        <span class="function-name">${func.name}</span>
                        <span class="function-line">Line ${func.start_line}</span>
                    </div>
                `).join('')}
                ${fileResult.functions.length > 5 ? `
                    <div class="function-item">
                        <span class="function-name">... and ${fileResult.functions.length - 5} more</span>
                    </div>
                ` : ''}
            </div>
        `;
        
        return card;
    }

    clearFiles() {
        this.files.clear();
        this.parsedFiles.clear();
        this.updateFileTree();
        this.updateFileSelectors();
        document.getElementById('parsingResults').style.display = 'none';
        document.getElementById('comparisonResults').style.display = 'none';
        document.getElementById('analysisResults').style.display = 'none';
        this.showToast('Files cleared', 'success');
    }

    loadSelectedFile(target) {
        const selectId = target === 'source' ? 'sourceFileSelect' : 'targetFileSelect';
        const editorId = target === 'source' ? 'sourceEditor' : 'targetEditor';
        
        const select = document.getElementById(selectId);
        const editor = document.getElementById(editorId);
        const textarea = editor.querySelector('textarea');
        
        const filename = select.value;
        if (!filename) {
            this.showToast('Please select a file first', 'warning');
            return;
        }
        
        const file = this.files.get(filename);
        if (file) {
            textarea.value = file.content;
            this.showToast(`Loaded ${filename}`, 'success');
        }
    }

    async compareFiles() {
        const sourceTextarea = document.querySelector('#sourceEditor textarea');
        const targetTextarea = document.querySelector('#targetEditor textarea');
        
        const sourceContent = sourceTextarea.value.trim();
        const targetContent = targetTextarea.value.trim();
        
        if (!sourceContent || !targetContent) {
            this.showToast('Please provide content for both files', 'warning');
            return;
        }
        
        this.showLoading('Comparing files...');
        
        try {
            const options = {
                threshold: parseFloat(document.getElementById('thresholdSlider').value),
                ignore_whitespace: document.getElementById('ignoreWhitespace').checked,
                detect_moves: document.getElementById('detectMoves').checked
            };
            
            const response = await fetch(`${this.apiBase}/compare`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    file1: {
                        path: 'source.c',
                        content: sourceContent
                    },
                    file2: {
                        path: 'target.c',
                        content: targetContent
                    },
                    options: options
                })
            });

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            const result = await response.json();
            this.displayComparisonResults(result);
            this.hideLoading();
            this.showToast('Comparison completed', 'success');
            
        } catch (error) {
            this.hideLoading();
            this.showToast(`Error comparing files: ${error.message}`, 'error');
        }
    }

    displayComparisonResults(result) {
        const resultsContainer = document.getElementById('comparisonResults');
        const summaryContainer = document.getElementById('resultsSummary');
        const diffContainer = document.getElementById('diffVisualization');
        
        resultsContainer.style.display = 'block';
        
        // Display summary
        const similarity = (result.similarity * 100).toFixed(1);
        const similarityColor = similarity > 80 ? 'var(--success-color)' : 
                               similarity > 50 ? 'var(--warning-color)' : 'var(--error-color)';
        
        summaryContainer.innerHTML = `
            <div class="comparison-summary">
                <div class="similarity-score" style="color: ${similarityColor}">
                    <h3>${similarity}% Similar</h3>
                    <p>Overall similarity between the files</p>
                </div>
                <div class="comparison-metrics">
                    <div class="metric">
                        <span class="metric-value">${result.analysis.changes.total_changes}</span>
                        <span class="metric-label">Changes</span>
                    </div>
                    <div class="metric">
                        <span class="metric-value">${result.analysis.functions.total_functions}</span>
                        <span class="metric-label">Functions</span>
                    </div>
                    <div class="metric">
                        <span class="metric-value">${result.execution_time_ms}ms</span>
                        <span class="metric-label">Time</span>
                    </div>
                </div>
            </div>
        `;
        
        // Display detailed changes
        diffContainer.innerHTML = `
            <div class="changes-list">
                <h4>📋 Detected Changes</h4>
                ${result.analysis.changes.detailed_changes.map(change => `
                    <div class="change-item">
                        <div class="change-header">
                            <span class="change-type ${change.change_type.toLowerCase()}">${change.change_type}</span>
                            <span class="change-confidence">${(change.confidence * 100).toFixed(0)}% confidence</span>
                        </div>
                        <div class="change-description">${change.description}</div>
                    </div>
                `).join('')}
            </div>
        `;
    }

    async runAnalysis() {
        if (this.files.size === 0) {
            this.showToast('No files to analyze', 'warning');
            return;
        }

        this.showLoading('Running advanced analysis...');
        
        try {
            const filesArray = Array.from(this.files.values()).map(file => ({
                path: file.path,
                content: file.content
            }));

            const options = {
                enable_cross_file_analysis: document.getElementById('crossFileAnalysis').checked,
                detect_duplicates: document.getElementById('detectDuplicates').checked,
                calculate_complexity: document.getElementById('calculateComplexity').checked
            };

            const response = await fetch(`${this.apiBase}/analyze`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    files: filesArray,
                    options: options
                })
            });

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            const result = await response.json();
            this.displayAnalysisResults(result);
            this.hideLoading();
            this.showToast('Analysis completed', 'success');
            
        } catch (error) {
            this.hideLoading();
            this.showToast(`Error running analysis: ${error.message}`, 'error');
        }
    }

    displayAnalysisResults(result) {
        const resultsContainer = document.getElementById('analysisResults');
        const dashboardContainer = document.getElementById('metricsDashboard');
        const detailsContainer = document.getElementById('analysisDetails');
        
        resultsContainer.style.display = 'block';
        
        // Display metrics dashboard
        dashboardContainer.innerHTML = `
            <div class="metrics-grid">
                <div class="metric-card">
                    <h4>📁 Project Overview</h4>
                    <div class="metric-list">
                        <div class="metric-item">
                            <span>Total Files:</span>
                            <span class="metric-value">${result.summary.total_files}</span>
                        </div>
                        <div class="metric-item">
                            <span>Total Functions:</span>
                            <span class="metric-value">${result.summary.total_functions}</span>
                        </div>
                        <div class="metric-item">
                            <span>Average Complexity:</span>
                            <span class="metric-value">${result.summary.average_complexity.toFixed(1)}</span>
                        </div>
                    </div>
                </div>
                
                <div class="metric-card">
                    <h4>🔍 Code Quality</h4>
                    <div class="metric-list">
                        <div class="metric-item">
                            <span>Duplicate Rate:</span>
                            <span class="metric-value">${(result.summary.duplicate_rate * 100).toFixed(1)}%</span>
                        </div>
                        <div class="metric-item">
                            <span>Dependencies:</span>
                            <span class="metric-value">${result.summary.dependency_count}</span>
                        </div>
                        <div class="metric-item">
                            <span>Duplicates Found:</span>
                            <span class="metric-value">${result.cross_file_analysis.duplicate_functions.length}</span>
                        </div>
                    </div>
                </div>
            </div>
        `;
        
        // Display detailed analysis
        if (result.cross_file_analysis.duplicate_functions.length > 0) {
            detailsContainer.innerHTML = `
                <div class="duplicates-section">
                    <h4>🔄 Duplicate Functions</h4>
                    <div class="duplicates-list">
                        ${result.cross_file_analysis.duplicate_functions.map(dup => `
                            <div class="duplicate-item">
                                <div class="duplicate-header">
                                    <span class="function-signature">${dup.signature}</span>
                                    <span class="similarity-badge">${(dup.similarity * 100).toFixed(0)}% similar</span>
                                </div>
                                <div class="duplicate-locations">
                                    ${dup.locations.map(loc => `
                                        <span class="location">📍 ${loc.function} (Line ${loc.start_line})</span>
                                    `).join('')}
                                </div>
                            </div>
                        `).join('')}
                    </div>
                </div>
            `;
        } else {
            detailsContainer.innerHTML = `
                <div class="no-duplicates">
                    <h4>✅ No duplicate functions found</h4>
                    <p>Your codebase appears to have good code reuse practices.</p>
                </div>
            `;
        }
    }

    async checkSystemHealth() {
        try {
            const response = await fetch(`${this.apiBase}/health`);
            const result = await response.json();
            
            const statusIndicator = document.getElementById('healthStatus');
            
            if (result.status === 'healthy') {
                statusIndicator.style.background = 'var(--success-color)';
                this.showToast(`System is healthy (v${result.version})`, 'success');
            } else {
                statusIndicator.style.background = 'var(--error-color)';
                this.showToast('System health check failed', 'error');
            }
            
        } catch (error) {
            const statusIndicator = document.getElementById('healthStatus');
            statusIndicator.style.background = 'var(--error-color)';
            this.showToast(`Health check failed: ${error.message}`, 'error');
        }
    }

    async saveSettings() {
        this.showLoading('Saving configuration...');
        
        try {
            const config = {
                parser: {
                    max_file_size: parseInt(document.getElementById('maxFileSize').value) * 1024 * 1024,
                    parse_timeout: parseInt(document.getElementById('parseTimeout').value),
                    enable_error_recovery: document.getElementById('enableErrorRecovery').checked
                },
                semantic: {
                    max_resolution_depth: parseInt(document.getElementById('maxResolutionDepth').value),
                    enable_cross_file_analysis: document.getElementById('enableCrossFileAnalysis').checked,
                    symbol_cache_size: parseInt(document.getElementById('symbolCacheSize').value)
                },
                diff_engine: {
                    default_similarity_threshold: parseFloat(document.getElementById('defaultThreshold').value),
                    enable_refactoring_detection: document.getElementById('enableRefactoringDetection').checked,
                    enable_cross_file_tracking: document.getElementById('enableCrossFileTracking').checked,
                    max_tree_depth: parseInt(document.getElementById('maxTreeDepth').value)
                }
            };

            const response = await fetch(`${this.apiBase}/configure`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(config)
            });

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            const result = await response.json();
            this.hideLoading();
            this.showToast(result.message, 'success');
            
            // Save to localStorage
            localStorage.setItem('smartDiffSettings', JSON.stringify(config));
            
        } catch (error) {
            this.hideLoading();
            this.showToast(`Error saving settings: ${error.message}`, 'error');
        }
    }

    loadSettings() {
        const saved = localStorage.getItem('smartDiffSettings');
        if (saved) {
            try {
                const config = JSON.parse(saved);
                
                // Load parser settings
                if (config.parser) {
                    document.getElementById('maxFileSize').value = (config.parser.max_file_size || 1048576) / (1024 * 1024);
                    document.getElementById('parseTimeout').value = config.parser.parse_timeout || 30;
                    document.getElementById('enableErrorRecovery').checked = config.parser.enable_error_recovery !== false;
                }
                
                // Load semantic settings
                if (config.semantic) {
                    document.getElementById('maxResolutionDepth').value = config.semantic.max_resolution_depth || 10;
                    document.getElementById('enableCrossFileAnalysis').checked = config.semantic.enable_cross_file_analysis !== false;
                    document.getElementById('symbolCacheSize').value = config.semantic.symbol_cache_size || 1000;
                }
                
                // Load diff engine settings
                if (config.diff_engine) {
                    document.getElementById('defaultThreshold').value = config.diff_engine.default_similarity_threshold || 0.8;
                    document.getElementById('enableRefactoringDetection').checked = config.diff_engine.enable_refactoring_detection !== false;
                    document.getElementById('enableCrossFileTracking').checked = config.diff_engine.enable_cross_file_tracking !== false;
                    document.getElementById('maxTreeDepth').value = config.diff_engine.max_tree_depth || 100;
                }
                
            } catch (error) {
                console.warn('Failed to load saved settings:', error);
            }
        }
    }

    resetSettings() {
        // Reset to defaults
        document.getElementById('maxFileSize').value = 1;
        document.getElementById('parseTimeout').value = 30;
        document.getElementById('enableErrorRecovery').checked = true;
        document.getElementById('maxResolutionDepth').value = 10;
        document.getElementById('enableCrossFileAnalysis').checked = true;
        document.getElementById('symbolCacheSize').value = 1000;
        document.getElementById('defaultThreshold').value = 0.8;
        document.getElementById('enableRefactoringDetection').checked = true;
        document.getElementById('enableCrossFileTracking').checked = true;
        document.getElementById('maxTreeDepth').value = 100;
        
        localStorage.removeItem('smartDiffSettings');
        this.showToast('Settings reset to defaults', 'success');
    }

    switchView(viewName) {
        // Update navigation
        document.querySelectorAll('.nav-btn').forEach(btn => {
            btn.classList.remove('active');
        });
        document.querySelector(`[data-view="${viewName}"]`).classList.add('active');
        
        // Update views
        document.querySelectorAll('.view').forEach(view => {
            view.classList.remove('active');
        });
        document.getElementById(viewName).classList.add('active');
        
        this.currentView = viewName;
    }

    showLoading(text = 'Processing...') {
        const overlay = document.getElementById('loadingOverlay');
        const loadingText = document.getElementById('loadingText');
        loadingText.textContent = text;
        overlay.style.display = 'flex';
    }

    hideLoading() {
        const overlay = document.getElementById('loadingOverlay');
        overlay.style.display = 'none';
    }

    showToast(message, type = 'info') {
        const container = document.getElementById('toastContainer');
        const toast = document.createElement('div');
        toast.className = `toast ${type}`;
        
        const icon = type === 'success' ? '✅' : 
                    type === 'error' ? '❌' : 
                    type === 'warning' ? '⚠️' : 'ℹ️';
        
        toast.innerHTML = `
            <div style="display: flex; align-items: center; gap: 8px;">
                <span>${icon}</span>
                <span>${message}</span>
            </div>
        `;
        
        container.appendChild(toast);
        
        // Auto remove after 5 seconds
        setTimeout(() => {
            if (toast.parentNode) {
                toast.parentNode.removeChild(toast);
            }
        }, 5000);
        
        // Remove on click
        toast.addEventListener('click', () => {
            if (toast.parentNode) {
                toast.parentNode.removeChild(toast);
            }
        });
    }
}

// Initialize the application when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.app = new SmartCodeDiffApp();
});
