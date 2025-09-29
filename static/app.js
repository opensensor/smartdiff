// Smart Code Diff - Advanced Web Application
class SmartCodeDiffApp {
    constructor() {
        this.files = new Map();
        this.parsedFiles = new Map();
        this.sourceBranch = new Map();
        this.targetBranch = new Map();
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

        // File inputs (these are created dynamically, so we don't need listeners here)

        // Branch selection buttons
        const sourceBranchBtn = document.getElementById('selectSourceBranchBtn');
        if (sourceBranchBtn) {
            sourceBranchBtn.addEventListener('click', () => {
                this.selectBranch('source');
            });
        }

        const targetBranchBtn = document.getElementById('selectTargetBranchBtn');
        if (targetBranchBtn) {
            targetBranchBtn.addEventListener('click', () => {
                this.selectBranch('target');
            });
        }

        // Explorer actions
        const selectDirectoryBtn = document.getElementById('selectDirectoryBtn');
        if (selectDirectoryBtn) {
            selectDirectoryBtn.addEventListener('click', () => {
                this.selectDirectory();
            });
        }

        const selectFilesBtn = document.getElementById('selectFilesBtn');
        if (selectFilesBtn) {
            selectFilesBtn.addEventListener('click', () => {
                this.selectFiles();
            });
        }

        const parseAllBtn = document.getElementById('parseAllBtn');
        if (parseAllBtn) {
            parseAllBtn.addEventListener('click', () => {
                this.parseAllFiles();
            });
        }

        const clearFilesBtn = document.getElementById('clearFilesBtn');
        if (clearFilesBtn) {
            clearFilesBtn.addEventListener('click', () => {
                this.clearFiles();
            });
        }

        // Compare actions
        const compareBtn = document.getElementById('compareBtn');
        if (compareBtn) {
            compareBtn.addEventListener('click', () => {
                this.compareFiles();
            });
        }

        const compareBranchesBtn = document.getElementById('compareBranchesBtn');
        if (compareBranchesBtn) {
            compareBranchesBtn.addEventListener('click', () => {
                this.compareBranches();
            });
        }

        const loadSourceBtn = document.getElementById('loadSourceBtn');
        if (loadSourceBtn) {
            loadSourceBtn.addEventListener('click', () => {
                this.loadSelectedFile('source');
            });
        }

        const loadTargetBtn = document.getElementById('loadTargetBtn');
        if (loadTargetBtn) {
            loadTargetBtn.addEventListener('click', () => {
                this.loadSelectedFile('target');
            });
        }

        // Analysis actions
        const runAnalysisBtn = document.getElementById('runAnalysisBtn');
        if (runAnalysisBtn) {
            runAnalysisBtn.addEventListener('click', () => {
                this.runAnalysis();
            });
        }

        // Settings actions
        const saveSettingsBtn = document.getElementById('saveSettingsBtn');
        if (saveSettingsBtn) {
            saveSettingsBtn.addEventListener('click', () => {
                this.saveSettings();
            });
        }

        const resetSettingsBtn = document.getElementById('resetSettingsBtn');
        if (resetSettingsBtn) {
            resetSettingsBtn.addEventListener('click', () => {
                this.resetSettings();
            });
        }

        // Health check
        const healthCheck = document.getElementById('healthCheck');
        if (healthCheck) {
            healthCheck.addEventListener('click', () => {
                this.checkSystemHealth();
            });
        }

        // Threshold slider
        const thresholdSlider = document.getElementById('thresholdSlider');
        if (thresholdSlider) {
            thresholdSlider.addEventListener('input', (e) => {
                const thresholdValue = document.getElementById('thresholdValue');
                if (thresholdValue) {
                    thresholdValue.textContent = e.target.value;
                }
            });
        }

        // Visualization tabs (will be set up when comparison results are shown)
        this.setupVisualizationTabs();

        // Graph controls (will be set up when graph is initialized)
        this.setupGraphControls();

        // Tree controls (will be set up when tree is initialized)
        this.setupTreeControls();
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
            this.selectFiles();
        });
    }

    setupVisualizationTabs() {
        // This will be called when comparison results are displayed
        setTimeout(() => {
            document.querySelectorAll('.tab-btn').forEach(btn => {
                btn.addEventListener('click', (e) => {
                    const tabName = e.currentTarget.dataset.tab;
                    this.switchVisualizationTab(tabName);
                });
            });
        }, 100);
    }

    setupGraphControls() {
        setTimeout(() => {
            const resetGraphBtn = document.getElementById('resetGraphBtn');
            if (resetGraphBtn) {
                resetGraphBtn.addEventListener('click', () => {
                    this.resetGraphView();
                });
            }

            const exportGraphBtn = document.getElementById('exportGraphBtn');
            if (exportGraphBtn) {
                exportGraphBtn.addEventListener('click', () => {
                    this.exportGraph();
                });
            }

            const graphLayout = document.getElementById('graphLayout');
            if (graphLayout) {
                graphLayout.addEventListener('change', (e) => {
                    this.updateGraphLayout(e.target.value);
                });
            }
        }, 100);
    }

    setupTreeControls() {
        setTimeout(() => {
            const expandAllBtn = document.getElementById('expandAllBtn');
            if (expandAllBtn) {
                expandAllBtn.addEventListener('click', () => {
                    this.expandAllTreeNodes();
                });
            }

            const collapseAllBtn = document.getElementById('collapseAllBtn');
            if (collapseAllBtn) {
                collapseAllBtn.addEventListener('click', () => {
                    this.collapseAllTreeNodes();
                });
            }

            const exportTreeBtn = document.getElementById('exportTreeBtn');
            if (exportTreeBtn) {
                exportTreeBtn.addEventListener('click', () => {
                    this.exportTree();
                });
            }
        }, 100);
    }

    selectDirectory() {
        console.log('Selecting directory...');

        // Create a fresh input element to ensure webkitdirectory works
        const input = document.createElement('input');
        input.type = 'file';
        input.webkitdirectory = true;
        input.multiple = true;
        input.style.display = 'none';

        console.log('Created directory input:', input);
        console.log('webkitdirectory set:', input.webkitdirectory);

        input.addEventListener('change', (e) => {
            console.log('Directory files selected:', e.target.files.length);
            this.handleFileSelection(e.target.files, true);
            if (input.parentNode) {
                document.body.removeChild(input);
            }
        });

        input.addEventListener('cancel', () => {
            console.log('Directory selection cancelled');
            if (input.parentNode) {
                document.body.removeChild(input);
            }
        });

        document.body.appendChild(input);

        // Add a small delay to ensure the element is in the DOM
        setTimeout(() => {
            input.click();
        }, 10);
    }

    selectFiles() {
        // Create a fresh input element for file selection
        const input = document.createElement('input');
        input.type = 'file';
        input.multiple = true;
        input.style.display = 'none';
        input.accept = '.c,.cpp,.cc,.cxx,.h,.hpp,.hxx,.js,.jsx,.ts,.tsx,.py,.java,.rs,.go,.rb,.php,.cs,.swift,.sh,.sql,.html,.css,.json,.yaml,.yml,.md,.txt';

        input.addEventListener('change', (e) => {
            this.handleFileSelection(e.target.files, false);
            document.body.removeChild(input);
        });

        document.body.appendChild(input);
        input.click();
    }

    selectBranch(branchType) {
        console.log(`Selecting ${branchType} branch...`);

        // Create a fresh input element for directory selection
        const input = document.createElement('input');
        input.type = 'file';
        input.webkitdirectory = true;
        input.multiple = true;
        input.style.display = 'none';

        console.log('Created input element:', input);
        console.log('webkitdirectory set:', input.webkitdirectory);

        input.addEventListener('change', (e) => {
            console.log(`Files selected for ${branchType}:`, e.target.files.length);
            this.handleBranchSelection(e.target.files, branchType);
            if (input.parentNode) {
                document.body.removeChild(input);
            }
        });

        input.addEventListener('cancel', () => {
            console.log('File selection cancelled');
            if (input.parentNode) {
                document.body.removeChild(input);
            }
        });

        document.body.appendChild(input);

        // Add a small delay to ensure the element is in the DOM
        setTimeout(() => {
            input.click();
        }, 10);
    }

    async handleDroppedItems(items) {
        const files = [];
        this.showLoading('Processing dropped items...');

        try {
            for (const item of items) {
                if (item.kind === 'file') {
                    const entry = item.webkitGetAsEntry();
                    if (entry) {
                        await this.traverseFileTree(entry, files);
                    }
                }
            }

            this.hideLoading();
            this.handleFileSelection(files, true);
        } catch (error) {
            this.hideLoading();
            this.showToast(`Error processing dropped items: ${error.message}`, 'error');
        }
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

        const loadingText = isDirectory ? 'Processing directory...' : 'Processing files...';
        this.showLoading(loadingText);

        try {
            let processedCount = 0;
            const totalFiles = Array.from(files).filter(file => this.isCodeFile(file.name)).length;

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

                    processedCount++;
                    this.showLoading(`${loadingText} (${processedCount}/${totalFiles})`);
                }
            }

            this.updateFileTree();
            this.updateFileSelectors();
            this.hideLoading();

            const message = isDirectory ?
                `Directory loaded: ${processedCount} code files found` :
                `${processedCount} files loaded successfully`;
            this.showToast(message, 'success');

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

    async handleBranchSelection(files, branchType) {
        if (!files || files.length === 0) return;

        const branchMap = branchType === 'source' ? this.sourceBranch : this.targetBranch;
        const branchNameElement = document.getElementById(`${branchType}BranchName`);

        this.showLoading(`Processing ${branchType} branch...`);

        try {
            branchMap.clear();
            let processedCount = 0;
            const totalFiles = Array.from(files).filter(file => this.isCodeFile(file.name)).length;

            // Get the root directory name from the first file
            const firstFile = files[0];
            const rootDir = firstFile.webkitRelativePath ? firstFile.webkitRelativePath.split('/')[0] : 'Selected Directory';

            for (const file of files) {
                if (this.isCodeFile(file.name)) {
                    const content = await this.readFileContent(file);
                    const relativePath = file.webkitRelativePath || file.name;

                    branchMap.set(relativePath, {
                        name: file.name,
                        path: relativePath,
                        content: content,
                        size: file.size,
                        lastModified: file.lastModified,
                        type: this.detectLanguage(file.name)
                    });

                    processedCount++;
                    this.showLoading(`Processing ${branchType} branch... (${processedCount}/${totalFiles})`);
                }
            }

            // Update branch name display
            branchNameElement.textContent = `${rootDir} (${processedCount} files)`;

            // Update file selectors for comparison
            this.updateBranchFileSelectors();

            // Show compare branches button if both branches are loaded
            if (this.sourceBranch.size > 0 && this.targetBranch.size > 0) {
                document.getElementById('compareBranchesBtn').style.display = 'block';
            }

            this.hideLoading();
            this.showToast(`${branchType} branch loaded: ${processedCount} files`, 'success');

        } catch (error) {
            this.hideLoading();
            this.showToast(`Error loading ${branchType} branch: ${error.message}`, 'error');
        }
    }

    updateBranchFileSelectors() {
        const sourceSelect = document.getElementById('sourceFileSelect');
        const targetSelect = document.getElementById('targetFileSelect');

        // Clear existing options
        sourceSelect.innerHTML = '<option value="">Select a file...</option>';
        targetSelect.innerHTML = '<option value="">Select a file...</option>';

        // Add source branch files
        for (const [path, file] of this.sourceBranch) {
            const option = document.createElement('option');
            option.value = `source:${path}`;
            option.textContent = `📁 ${file.path} (${file.type})`;
            sourceSelect.appendChild(option);
        }

        // Add target branch files
        for (const [path, file] of this.targetBranch) {
            const option = document.createElement('option');
            option.value = `target:${path}`;
            option.textContent = `📁 ${file.path} (${file.type})`;
            targetSelect.appendChild(option);
        }

        // Also add regular files if any
        for (const [name, file] of this.files) {
            const option1 = document.createElement('option');
            option1.value = `regular:${name}`;
            option1.textContent = `📄 ${file.name} (${file.type})`;
            sourceSelect.appendChild(option1);

            const option2 = document.createElement('option');
            option2.value = `regular:${name}`;
            option2.textContent = `📄 ${file.name} (${file.type})`;
            targetSelect.appendChild(option2);
        }
    }

    loadSelectedFile(target) {
        const selectId = target === 'source' ? 'sourceFileSelect' : 'targetFileSelect';
        const editorId = target === 'source' ? 'sourceEditor' : 'targetEditor';
        const pathId = target === 'source' ? 'sourceFilePath' : 'targetFilePath';

        const select = document.getElementById(selectId);
        const editor = document.getElementById(editorId);
        const textarea = editor.querySelector('textarea');
        const pathDisplay = document.getElementById(pathId);

        const selection = select.value;
        if (!selection) {
            this.showToast('Please select a file first', 'warning');
            return;
        }

        const [source, path] = selection.split(':', 2);
        let file = null;

        if (source === 'source') {
            file = this.sourceBranch.get(path);
        } else if (source === 'target') {
            file = this.targetBranch.get(path);
        } else if (source === 'regular') {
            file = this.files.get(path);
        }

        if (file) {
            textarea.value = file.content;
            pathDisplay.textContent = file.path;
            this.showToast(`Loaded ${file.name}`, 'success');
        } else {
            this.showToast('File not found', 'error');
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

    async compareBranches() {
        if (this.sourceBranch.size === 0 || this.targetBranch.size === 0) {
            this.showToast('Please load both source and target branches first', 'warning');
            return;
        }

        this.showLoading('Comparing branches...');

        try {
            // Find common files between branches
            const commonFiles = [];
            const sourceFiles = new Set(Array.from(this.sourceBranch.keys()).map(path => path.split('/').pop()));

            for (const [targetPath, targetFile] of this.targetBranch) {
                const targetFileName = targetPath.split('/').pop();
                if (sourceFiles.has(targetFileName)) {
                    // Find the corresponding source file
                    for (const [sourcePath, sourceFile] of this.sourceBranch) {
                        const sourceFileName = sourcePath.split('/').pop();
                        if (sourceFileName === targetFileName) {
                            commonFiles.push({
                                name: targetFileName,
                                sourcePath: sourcePath,
                                targetPath: targetPath,
                                sourceFile: sourceFile,
                                targetFile: targetFile
                            });
                            break;
                        }
                    }
                }
            }

            if (commonFiles.length === 0) {
                this.hideLoading();
                this.showToast('No common files found between branches', 'warning');
                return;
            }

            // Compare all common files
            const comparisons = [];
            for (const commonFile of commonFiles) {
                const options = {
                    threshold: parseFloat(document.getElementById('thresholdSlider')?.value || 0.8),
                    ignore_whitespace: document.getElementById('ignoreWhitespace')?.checked || false,
                    detect_moves: document.getElementById('detectMoves')?.checked || true
                };

                const response = await fetch(`${this.apiBase}/compare`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        file1: {
                            path: commonFile.sourcePath,
                            content: commonFile.sourceFile.content
                        },
                        file2: {
                            path: commonFile.targetPath,
                            content: commonFile.targetFile.content
                        },
                        options: options
                    })
                });

                if (!response.ok) {
                    throw new Error(`HTTP ${response.status}: ${response.statusText}`);
                }

                const result = await response.json();
                comparisons.push({
                    ...commonFile,
                    comparison: result
                });
            }

            this.displayBranchComparisonResults(comparisons);
            this.hideLoading();
            this.showToast(`Compared ${comparisons.length} files between branches`, 'success');

        } catch (error) {
            this.hideLoading();
            this.showToast(`Error comparing branches: ${error.message}`, 'error');
        }
    }

    displayBranchComparisonResults(comparisons) {
        const resultsContainer = document.getElementById('comparisonResults');
        const summaryContainer = document.getElementById('resultsSummary');
        const diffContainer = document.getElementById('diffVisualization');

        resultsContainer.style.display = 'block';

        // Calculate overall statistics
        const totalFiles = comparisons.length;
        const avgSimilarity = comparisons.reduce((sum, comp) => sum + comp.comparison.similarity, 0) / totalFiles;
        const totalChanges = comparisons.reduce((sum, comp) => sum + comp.comparison.analysis.changes.total_changes, 0);

        // Display summary
        const similarityPercent = (avgSimilarity * 100).toFixed(1);
        const similarityColor = avgSimilarity > 0.8 ? 'var(--success-color)' :
                               avgSimilarity > 0.5 ? 'var(--warning-color)' : 'var(--error-color)';

        summaryContainer.innerHTML = `
            <div class="comparison-summary">
                <div class="similarity-score" style="color: ${similarityColor}">
                    <h3>${similarityPercent}% Similar</h3>
                    <p>Average similarity across ${totalFiles} files</p>
                </div>
                <div class="comparison-metrics">
                    <div class="metric">
                        <span class="metric-value">${totalFiles}</span>
                        <span class="metric-label">Files Compared</span>
                    </div>
                    <div class="metric">
                        <span class="metric-value">${totalChanges}</span>
                        <span class="metric-label">Total Changes</span>
                    </div>
                    <div class="metric">
                        <span class="metric-value">${comparisons.filter(c => c.comparison.similarity < 0.9).length}</span>
                        <span class="metric-label">Modified Files</span>
                    </div>
                </div>
            </div>
        `;

        // Display detailed file comparisons
        diffContainer.innerHTML = `
            <div class="branch-comparison-results">
                <h4>📊 File-by-File Comparison</h4>
                <div class="file-comparisons">
                    ${comparisons.map(comp => {
                        const similarity = (comp.comparison.similarity * 100).toFixed(1);
                        const similarityClass = comp.comparison.similarity > 0.9 ? 'high' :
                                              comp.comparison.similarity > 0.7 ? 'medium' : 'low';

                        return `
                            <div class="file-comparison-item">
                                <div class="file-comparison-header">
                                    <span class="file-name">${comp.name}</span>
                                    <span class="similarity-badge ${similarityClass}">${similarity}%</span>
                                </div>
                                <div class="file-paths">
                                    <div class="path-item">
                                        <span class="path-label">Source:</span>
                                        <span class="path-value">${comp.sourcePath}</span>
                                    </div>
                                    <div class="path-item">
                                        <span class="path-label">Target:</span>
                                        <span class="path-value">${comp.targetPath}</span>
                                    </div>
                                </div>
                                <div class="file-changes">
                                    <span class="changes-count">${comp.comparison.analysis.changes.total_changes} changes</span>
                                    ${comp.comparison.analysis.changes.detailed_changes.slice(0, 3).map(change => `
                                        <span class="change-tag ${change.change_type.toLowerCase()}">${change.change_type}</span>
                                    `).join('')}
                                    ${comp.comparison.analysis.changes.detailed_changes.length > 3 ?
                                        `<span class="more-changes">+${comp.comparison.analysis.changes.detailed_changes.length - 3} more</span>` : ''}
                                </div>
                            </div>
                        `;
                    }).join('')}
                </div>
            </div>
        `;
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

        // Display detailed changes in text diff tab
        const textDiffContainer = document.getElementById('diffVisualization');
        if (textDiffContainer) {
            textDiffContainer.innerHTML = `
                <div class="changes-list">
                    <h4>📋 Detected Changes</h4>
                    ${result.analysis && result.analysis.changes && result.analysis.changes.detailed_changes ?
                        result.analysis.changes.detailed_changes.map(change => `
                            <div class="change-item">
                                <div class="change-header">
                                    <span class="change-type ${change.change_type.toLowerCase()}">${change.change_type}</span>
                                    <span class="change-confidence">${(change.confidence * 100).toFixed(0)}% confidence</span>
                                </div>
                                <div class="change-description">${change.description}</div>
                            </div>
                        `).join('') :
                        '<p>No detailed changes available</p>'
                    }
                </div>
                <div class="code-diff-container">
                    <h4>📝 Code Differences</h4>
                    <div class="diff-sections">
                        ${this.generateCodeDiff(result)}
                    </div>
                </div>
            `;
        }

        // Set up visualization tabs
        this.setupVisualizationTabs();
        this.setupGraphControls();
        this.setupTreeControls();

        // Generate graph visualization if enabled
        if (document.getElementById('enableGraphView')?.checked) {
            this.generateFunctionGraph(result);
        }

        // Generate AST tree visualization
        this.generateASTTree(result);
    }

    generateCodeDiff(result) {
        // Get the source and target code from the editors
        const sourceCode = document.querySelector('#sourceEditor textarea')?.value || '';
        const targetCode = document.querySelector('#targetEditor textarea')?.value || '';

        if (!sourceCode || !targetCode) {
            return '<p>No source or target code available for diff visualization</p>';
        }

        // Simple line-by-line diff
        const sourceLines = sourceCode.split('\n');
        const targetLines = targetCode.split('\n');

        return `
            <div class="side-by-side-diff">
                <div class="diff-side">
                    <h5>Source (Original)</h5>
                    <div class="code-lines">
                        ${sourceLines.map((line, index) => `
                            <div class="code-line" data-line="${index + 1}">
                                <span class="line-number">${index + 1}</span>
                                <span class="line-content">${this.escapeHtml(line)}</span>
                            </div>
                        `).join('')}
                    </div>
                </div>
                <div class="diff-side">
                    <h5>Target (Modified)</h5>
                    <div class="code-lines">
                        ${targetLines.map((line, index) => `
                            <div class="code-line" data-line="${index + 1}">
                                <span class="line-number">${index + 1}</span>
                                <span class="line-content">${this.escapeHtml(line)}</span>
                            </div>
                        `).join('')}
                    </div>
                </div>
            </div>
        `;
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    switchVisualizationTab(tabName) {
        // Update tab buttons
        document.querySelectorAll('.tab-btn').forEach(btn => {
            btn.classList.remove('active');
        });
        document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');

        // Update tab content
        document.querySelectorAll('.tab-content').forEach(content => {
            content.classList.remove('active');
        });
        document.getElementById(`${tabName}Tab`).classList.add('active');

        // Initialize visualization if needed
        if (tabName === 'graph' && !this.graphInitialized) {
            this.initializeGraph();
        }
    }

    generateFunctionGraph(comparisonResult) {
        if (!window.d3) {
            console.warn('D3.js not loaded, skipping graph visualization');
            return;
        }

        const functions = this.extractFunctionsFromComparison(comparisonResult);
        const nodes = functions.map(func => ({
            id: func.name,
            name: func.name,
            type: func.changeType,
            similarity: func.similarity,
            sourceLines: func.sourceLines,
            targetLines: func.targetLines,
            complexity: func.complexity || 1
        }));

        const links = this.generateFunctionLinks(functions);

        this.renderFunctionGraph(nodes, links);
    }

    extractFunctionsFromComparison(result) {
        const functions = [];

        // Extract from detailed changes
        if (result.analysis && result.analysis.changes && result.analysis.changes.detailed_changes) {
            result.analysis.changes.detailed_changes.forEach(change => {
                if (change.description.includes('function')) {
                    const functionName = this.extractFunctionName(change.description);
                    if (functionName) {
                        functions.push({
                            name: functionName,
                            changeType: change.change_type.toLowerCase(),
                            similarity: change.confidence,
                            description: change.description
                        });
                    }
                }
            });
        }

        // Add some mock functions for demonstration if no functions found
        if (functions.length === 0) {
            functions.push(
                { name: 'main', changeType: 'modify', similarity: 0.85, sourceLines: [1, 20], targetLines: [1, 25] },
                { name: 'init_system', changeType: 'unchanged', similarity: 1.0, sourceLines: [22, 45], targetLines: [27, 50] },
                { name: 'process_data', changeType: 'modify', similarity: 0.72, sourceLines: [47, 80], targetLines: [52, 90] },
                { name: 'cleanup', changeType: 'add', similarity: 0.0, sourceLines: null, targetLines: [92, 105] },
                { name: 'old_handler', changeType: 'delete', similarity: 0.0, sourceLines: [82, 95], targetLines: null }
            );
        }

        return functions;
    }

    extractFunctionName(description) {
        // Try to extract function name from description
        const patterns = [
            /function\s+(\w+)/i,
            /(\w+)\s+function/i,
            /`(\w+)`/,
            /'(\w+)'/,
            /"(\w+)"/
        ];

        for (const pattern of patterns) {
            const match = description.match(pattern);
            if (match) {
                return match[1];
            }
        }

        return null;
    }

    generateFunctionLinks(functions) {
        const links = [];

        // Generate links based on function call relationships (simplified)
        for (let i = 0; i < functions.length; i++) {
            for (let j = i + 1; j < functions.length; j++) {
                const func1 = functions[i];
                const func2 = functions[j];

                // Create links based on naming patterns or proximity
                if (this.shouldLinkFunctions(func1, func2)) {
                    links.push({
                        source: func1.name,
                        target: func2.name,
                        type: 'calls',
                        strength: this.calculateLinkStrength(func1, func2)
                    });
                }
            }
        }

        return links;
    }

    shouldLinkFunctions(func1, func2) {
        // Simple heuristics for linking functions
        if (func1.name === 'main') return true;
        if (func2.name === 'main') return true;
        if (func1.name.includes('init') && func2.name.includes('process')) return true;
        if (func1.name.includes('process') && func2.name.includes('cleanup')) return true;

        return Math.random() > 0.7; // Random links for demonstration
    }

    calculateLinkStrength(func1, func2) {
        // Calculate link strength based on function similarity and types
        if (func1.changeType === func2.changeType) return 0.8;
        if (func1.changeType === 'unchanged' || func2.changeType === 'unchanged') return 0.6;
        return 0.4;
    }

    renderFunctionGraph(nodes, links) {
        const container = document.getElementById('functionGraph');
        const width = container.clientWidth || 800;
        const height = 600;

        // Clear previous graph
        d3.select('#functionGraph').selectAll('*').remove();

        const svg = d3.select('#functionGraph')
            .attr('width', width)
            .attr('height', height);

        // Create zoom behavior
        const zoom = d3.zoom()
            .scaleExtent([0.1, 4])
            .on('zoom', (event) => {
                g.attr('transform', event.transform);
            });

        svg.call(zoom);

        const g = svg.append('g');

        // Create force simulation
        const simulation = d3.forceSimulation(nodes)
            .force('link', d3.forceLink(links).id(d => d.id).distance(100))
            .force('charge', d3.forceManyBody().strength(-300))
            .force('center', d3.forceCenter(width / 2, height / 2))
            .force('collision', d3.forceCollide().radius(30));

        // Create links
        const link = g.append('g')
            .selectAll('line')
            .data(links)
            .enter().append('line')
            .attr('class', 'graph-link')
            .attr('stroke-width', d => Math.sqrt(d.strength * 5));

        // Create nodes
        const node = g.append('g')
            .selectAll('circle')
            .data(nodes)
            .enter().append('circle')
            .attr('class', 'graph-node')
            .attr('r', d => 8 + (d.complexity || 1) * 2)
            .attr('fill', d => this.getNodeColor(d.type))
            .attr('stroke', '#fff')
            .attr('stroke-width', 2)
            .call(d3.drag()
                .on('start', (event, d) => {
                    if (!event.active) simulation.alphaTarget(0.3).restart();
                    d.fx = d.x;
                    d.fy = d.y;
                })
                .on('drag', (event, d) => {
                    d.fx = event.x;
                    d.fy = event.y;
                })
                .on('end', (event, d) => {
                    if (!event.active) simulation.alphaTarget(0);
                    d.fx = null;
                    d.fy = null;
                }));

        // Add labels
        const label = g.append('g')
            .selectAll('text')
            .data(nodes)
            .enter().append('text')
            .attr('class', 'graph-label')
            .text(d => d.name)
            .attr('dy', -15);

        // Add tooltips
        node.on('mouseover', (event, d) => {
            this.showGraphTooltip(event, d);
        }).on('mouseout', () => {
            this.hideGraphTooltip();
        });

        // Update positions on simulation tick
        simulation.on('tick', () => {
            link
                .attr('x1', d => d.source.x)
                .attr('y1', d => d.source.y)
                .attr('x2', d => d.target.x)
                .attr('y2', d => d.target.y);

            node
                .attr('cx', d => d.x)
                .attr('cy', d => d.y);

            label
                .attr('x', d => d.x)
                .attr('y', d => d.y);
        });

        // Store simulation for controls
        this.graphSimulation = simulation;
        this.graphInitialized = true;
    }

    getNodeColor(changeType) {
        const colors = {
            'add': '#22c55e',
            'delete': '#ef4444',
            'modify': '#f59e0b',
            'unchanged': '#6b7280'
        };
        return colors[changeType] || '#6b7280';
    }

    showGraphTooltip(event, data) {
        const tooltip = d3.select('body').append('div')
            .attr('class', 'graph-tooltip')
            .style('opacity', 0);

        const content = `
            <strong>${data.name}</strong><br/>
            Type: ${data.type}<br/>
            Similarity: ${(data.similarity * 100).toFixed(1)}%<br/>
            ${data.sourceLines ? `Source: Lines ${data.sourceLines[0]}-${data.sourceLines[1]}` : ''}<br/>
            ${data.targetLines ? `Target: Lines ${data.targetLines[0]}-${data.targetLines[1]}` : ''}
        `;

        tooltip.html(content)
            .style('left', (event.pageX + 10) + 'px')
            .style('top', (event.pageY - 10) + 'px')
            .transition()
            .duration(200)
            .style('opacity', 1);

        this.currentTooltip = tooltip;
    }

    hideGraphTooltip() {
        if (this.currentTooltip) {
            this.currentTooltip.transition()
                .duration(200)
                .style('opacity', 0)
                .remove();
            this.currentTooltip = null;
        }
    }

    resetGraphView() {
        if (this.graphSimulation) {
            this.graphSimulation.alpha(1).restart();
        }

        // Reset zoom
        const svg = d3.select('#functionGraph');
        svg.transition().duration(750).call(
            d3.zoom().transform,
            d3.zoomIdentity
        );
    }

    updateGraphLayout(layoutType) {
        if (!this.graphSimulation) return;

        // Update forces based on layout type
        switch (layoutType) {
            case 'hierarchical':
                this.graphSimulation
                    .force('charge', d3.forceManyBody().strength(-100))
                    .force('link', d3.forceLink().distance(80))
                    .force('y', d3.forceY().strength(0.1));
                break;
            case 'circular':
                this.graphSimulation
                    .force('charge', d3.forceManyBody().strength(-50))
                    .force('link', d3.forceLink().distance(60))
                    .force('radial', d3.forceRadial(150, 400, 300).strength(0.1));
                break;
            default: // force-directed
                this.graphSimulation
                    .force('charge', d3.forceManyBody().strength(-300))
                    .force('link', d3.forceLink().distance(100))
                    .force('y', null)
                    .force('radial', null);
        }

        this.graphSimulation.alpha(1).restart();
    }

    exportGraph() {
        const svg = document.getElementById('functionGraph');
        const serializer = new XMLSerializer();
        const svgString = serializer.serializeToString(svg);

        const blob = new Blob([svgString], { type: 'image/svg+xml' });
        const url = URL.createObjectURL(blob);

        const a = document.createElement('a');
        a.href = url;
        a.download = 'function-graph.svg';
        a.click();

        URL.revokeObjectURL(url);
    }

    generateASTTree(comparisonResult) {
        // Try to extract real AST data from the comparison result
        let sourceAST, targetAST;

        if (comparisonResult.source_ast && comparisonResult.target_ast) {
            // Use real AST data if available
            sourceAST = comparisonResult.source_ast;
            targetAST = comparisonResult.target_ast;
        } else if (comparisonResult.analysis && comparisonResult.analysis.functions) {
            // Generate AST from function analysis
            sourceAST = this.generateASTFromFunctions(comparisonResult.analysis.functions, 'source');
            targetAST = this.generateASTFromFunctions(comparisonResult.analysis.functions, 'target');
        } else {
            // Fallback to mock data with a note
            sourceAST = this.generateMockAST('source');
            targetAST = this.generateMockAST('target');

            // Add a note about mock data
            const noteElement = document.createElement('div');
            noteElement.className = 'ast-note';
            noteElement.innerHTML = '⚠️ <strong>Note:</strong> AST data not available from API. Showing example structure.';

            const sourceContainer = document.getElementById('sourceTreeContent');
            const targetContainer = document.getElementById('targetTreeContent');

            if (sourceContainer) sourceContainer.prepend(noteElement.cloneNode(true));
            if (targetContainer) targetContainer.prepend(noteElement.cloneNode(true));
        }

        this.renderASTTree(sourceAST, 'sourceTreeContent');
        this.renderASTTree(targetAST, 'targetTreeContent');
    }

    generateASTFromFunctions(functionsData, type) {
        // Convert function analysis data to AST-like structure
        const functions = functionsData.detailed_functions || [];

        return {
            type: 'translation_unit',
            children: functions.map(func => ({
                type: 'function_definition',
                name: func.name || 'unknown_function',
                changeType: this.determineFunctionChangeType(func, type),
                children: [
                    {
                        type: 'compound_statement',
                        children: [
                            {
                                type: 'declaration',
                                name: `Parameters: ${func.parameters || 'none'}`,
                                changeType: 'unchanged'
                            },
                            {
                                type: 'expression_statement',
                                name: `Lines: ${func.start_line || 0}-${func.end_line || 0}`,
                                changeType: 'unchanged'
                            },
                            {
                                type: 'return_statement',
                                name: `Complexity: ${func.complexity || 1}`,
                                changeType: 'unchanged'
                            }
                        ]
                    }
                ]
            }))
        };
    }

    determineFunctionChangeType(func, type) {
        // Simple heuristic to determine change type
        if (func.similarity !== undefined) {
            if (func.similarity > 0.9) return 'unchanged';
            if (func.similarity > 0.5) return 'modify';
            return type === 'source' ? 'delete' : 'add';
        }
        return 'unchanged';
    }

    generateMockAST(type) {
        // Generate a mock AST structure for demonstration
        return {
            type: 'translation_unit',
            children: [
                {
                    type: 'function_definition',
                    name: 'main',
                    changeType: type === 'source' ? 'modify' : 'modify',
                    children: [
                        {
                            type: 'compound_statement',
                            children: [
                                {
                                    type: 'declaration',
                                    name: 'int x = 10',
                                    changeType: 'unchanged'
                                },
                                {
                                    type: 'expression_statement',
                                    name: 'printf("Hello")',
                                    changeType: type === 'source' ? 'delete' : null
                                },
                                {
                                    type: 'expression_statement',
                                    name: 'printf("Hello World")',
                                    changeType: type === 'target' ? 'add' : null
                                },
                                {
                                    type: 'return_statement',
                                    name: 'return 0',
                                    changeType: 'unchanged'
                                }
                            ]
                        }
                    ]
                },
                {
                    type: 'function_definition',
                    name: 'init_system',
                    changeType: 'unchanged',
                    children: [
                        {
                            type: 'compound_statement',
                            children: [
                                {
                                    type: 'declaration',
                                    name: 'int status = 0',
                                    changeType: 'unchanged'
                                }
                            ]
                        }
                    ]
                }
            ]
        };
    }

    renderASTTree(ast, containerId) {
        const container = document.getElementById(containerId);
        container.innerHTML = '';

        const treeElement = this.createTreeNode(ast, 0);
        container.appendChild(treeElement);
    }

    createTreeNode(node, depth) {
        const nodeElement = document.createElement('div');
        nodeElement.className = `tree-node ${node.changeType || ''}`;
        nodeElement.style.marginLeft = `${depth * 20}px`;

        const content = document.createElement('div');
        content.className = 'tree-node-content';

        const icon = document.createElement('span');
        icon.className = 'tree-node-icon';
        icon.textContent = node.children && node.children.length > 0 ? '📁' : '📄';

        const label = document.createElement('span');
        label.className = 'tree-node-label';
        label.textContent = node.name || node.type;

        const type = document.createElement('span');
        type.className = 'tree-node-type';
        type.textContent = node.type;

        content.appendChild(icon);
        content.appendChild(label);
        content.appendChild(type);
        nodeElement.appendChild(content);

        // Add click handler for expansion
        if (node.children && node.children.length > 0) {
            let expanded = true;
            const childrenContainer = document.createElement('div');
            childrenContainer.className = 'tree-children';

            node.children.forEach(child => {
                if (child) { // Only render non-null children
                    const childElement = this.createTreeNode(child, depth + 1);
                    childrenContainer.appendChild(childElement);
                }
            });

            nodeElement.appendChild(childrenContainer);

            content.addEventListener('click', () => {
                expanded = !expanded;
                childrenContainer.style.display = expanded ? 'block' : 'none';
                icon.textContent = expanded ? '📁' : '📂';
                nodeElement.classList.toggle('expanded', expanded);
            });
        }

        return nodeElement;
    }

    expandAllTreeNodes() {
        document.querySelectorAll('.tree-children').forEach(children => {
            children.style.display = 'block';
        });
        document.querySelectorAll('.tree-node-icon').forEach(icon => {
            if (icon.textContent === '📂') {
                icon.textContent = '📁';
            }
        });
        document.querySelectorAll('.tree-node').forEach(node => {
            node.classList.add('expanded');
        });
    }

    collapseAllTreeNodes() {
        document.querySelectorAll('.tree-children').forEach(children => {
            children.style.display = 'none';
        });
        document.querySelectorAll('.tree-node-icon').forEach(icon => {
            if (icon.textContent === '📁') {
                icon.textContent = '📂';
            }
        });
        document.querySelectorAll('.tree-node').forEach(node => {
            node.classList.remove('expanded');
        });
    }

    exportTree() {
        const sourceTree = document.getElementById('sourceTreeContent').innerHTML;
        const targetTree = document.getElementById('targetTreeContent').innerHTML;

        const html = `
            <!DOCTYPE html>
            <html>
            <head>
                <title>AST Tree Comparison</title>
                <style>
                    body { font-family: monospace; }
                    .tree-container { display: flex; }
                    .tree-side { flex: 1; padding: 20px; }
                    .tree-node { margin: 2px 0; padding: 2px 5px; }
                    .tree-node.added { background: rgba(34, 197, 94, 0.1); }
                    .tree-node.removed { background: rgba(239, 68, 68, 0.1); }
                    .tree-node.modified { background: rgba(245, 158, 11, 0.1); }
                </style>
            </head>
            <body>
                <h1>AST Tree Comparison</h1>
                <div class="tree-container">
                    <div class="tree-side">
                        <h2>Source</h2>
                        ${sourceTree}
                    </div>
                    <div class="tree-side">
                        <h2>Target</h2>
                        ${targetTree}
                    </div>
                </div>
            </body>
            </html>
        `;

        const blob = new Blob([html], { type: 'text/html' });
        const url = URL.createObjectURL(blob);

        const a = document.createElement('a');
        a.href = url;
        a.download = 'ast-tree-comparison.html';
        a.click();

        URL.revokeObjectURL(url);
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
