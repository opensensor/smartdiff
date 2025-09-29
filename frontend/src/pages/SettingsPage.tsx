import React, { useState } from 'react';
import { 
  Settings, 
  Save, 
  RotateCcw, 
  CheckCircle, 
  AlertCircle,
  Info,
  Sliders,
  Code,
  Eye,
  Zap
} from 'lucide-react';
import { clsx } from 'clsx';

interface AppSettings {
  parser: {
    maxFileSize: number;
    parseTimeout: number;
    enableErrorRecovery: boolean;
  };
  semantic: {
    maxResolutionDepth: number;
    enableCrossFileAnalysis: boolean;
    symbolCacheSize: number;
  };
  diffEngine: {
    defaultSimilarityThreshold: number;
    enableRefactoringDetection: boolean;
    enableCrossFileTracking: boolean;
    maxTreeDepth: number;
  };
  output: {
    defaultFormat: string;
    enableColors: boolean;
    includeTimestamps: boolean;
  };
  ui: {
    theme: string;
    showLineNumbers: boolean;
    enableSyntaxHighlighting: boolean;
    autoSave: boolean;
  };
}

export const SettingsPage: React.FC = () => {
  const [settings, setSettings] = useState<AppSettings>({
    parser: {
      maxFileSize: 10485760, // 10MB
      parseTimeout: 30,
      enableErrorRecovery: true,
    },
    semantic: {
      maxResolutionDepth: 10,
      enableCrossFileAnalysis: true,
      symbolCacheSize: 1000,
    },
    diffEngine: {
      defaultSimilarityThreshold: 0.7,
      enableRefactoringDetection: true,
      enableCrossFileTracking: true,
      maxTreeDepth: 20,
    },
    output: {
      defaultFormat: 'text',
      enableColors: true,
      includeTimestamps: false,
    },
    ui: {
      theme: 'light',
      showLineNumbers: true,
      enableSyntaxHighlighting: true,
      autoSave: true,
    },
  });

  const [activeTab, setActiveTab] = useState('parser');
  const [saveStatus, setSaveStatus] = useState<'idle' | 'saving' | 'saved' | 'error'>('idle');

  const tabs = [
    { id: 'parser', name: 'Parser', icon: Code },
    { id: 'semantic', name: 'Semantic', icon: Sliders },
    { id: 'diffEngine', name: 'Diff Engine', icon: Zap },
    { id: 'output', name: 'Output', icon: Eye },
    { id: 'ui', name: 'Interface', icon: Settings },
  ];

  const handleSave = async () => {
    setSaveStatus('saving');
    
    try {
      // Simulate API call to save settings
      await new Promise(resolve => setTimeout(resolve, 1000));
      setSaveStatus('saved');
      setTimeout(() => setSaveStatus('idle'), 2000);
    } catch (error) {
      setSaveStatus('error');
      setTimeout(() => setSaveStatus('idle'), 3000);
    }
  };

  const handleReset = () => {
    // Reset to default values
    setSettings({
      parser: {
        maxFileSize: 10485760,
        parseTimeout: 30,
        enableErrorRecovery: true,
      },
      semantic: {
        maxResolutionDepth: 10,
        enableCrossFileAnalysis: true,
        symbolCacheSize: 1000,
      },
      diffEngine: {
        defaultSimilarityThreshold: 0.7,
        enableRefactoringDetection: true,
        enableCrossFileTracking: true,
        maxTreeDepth: 20,
      },
      output: {
        defaultFormat: 'text',
        enableColors: true,
        includeTimestamps: false,
      },
      ui: {
        theme: 'light',
        showLineNumbers: true,
        enableSyntaxHighlighting: true,
        autoSave: true,
      },
    });
  };

  const updateSettings = (section: keyof AppSettings, key: string, value: any) => {
    setSettings(prev => ({
      ...prev,
      [section]: {
        ...prev[section],
        [key]: value,
      },
    }));
  };

  const formatFileSize = (bytes: number) => {
    const mb = bytes / (1024 * 1024);
    return `${mb.toFixed(1)} MB`;
  };

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          Settings
        </h1>
        <p className="text-gray-600">
          Configure the Smart Code Diff tool to match your preferences and requirements.
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-8">
        {/* Sidebar Navigation */}
        <div className="lg:col-span-1">
          <nav className="space-y-1">
            {tabs.map((tab) => {
              const Icon = tab.icon;
              return (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={clsx(
                    'w-full flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors',
                    activeTab === tab.id
                      ? 'bg-primary-100 text-primary-700 border-r-2 border-primary-500'
                      : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'
                  )}
                >
                  <Icon className="h-5 w-5 mr-3" />
                  {tab.name}
                </button>
              );
            })}
          </nav>
        </div>

        {/* Settings Content */}
        <div className="lg:col-span-3">
          <div className="card p-6">
            {/* Parser Settings */}
            {activeTab === 'parser' && (
              <div className="space-y-6">
                <div>
                  <h3 className="text-lg font-medium text-gray-900 mb-4">
                    Parser Configuration
                  </h3>
                  <div className="space-y-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        Maximum File Size: {formatFileSize(settings.parser.maxFileSize)}
                      </label>
                      <input
                        type="range"
                        min="1048576"
                        max="104857600"
                        step="1048576"
                        value={settings.parser.maxFileSize}
                        onChange={(e) => updateSettings('parser', 'maxFileSize', parseInt(e.target.value))}
                        className="w-full"
                      />
                      <div className="flex justify-between text-xs text-gray-500 mt-1">
                        <span>1 MB</span>
                        <span>100 MB</span>
                      </div>
                    </div>

                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        Parse Timeout (seconds)
                      </label>
                      <input
                        type="number"
                        min="5"
                        max="300"
                        value={settings.parser.parseTimeout}
                        onChange={(e) => updateSettings('parser', 'parseTimeout', parseInt(e.target.value))}
                        className="input w-full"
                      />
                    </div>

                    <div>
                      <label className="flex items-center">
                        <input
                          type="checkbox"
                          checked={settings.parser.enableErrorRecovery}
                          onChange={(e) => updateSettings('parser', 'enableErrorRecovery', e.target.checked)}
                          className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                        />
                        <span className="ml-2 text-sm text-gray-700">
                          Enable syntax error recovery
                        </span>
                      </label>
                      <p className="text-xs text-gray-500 mt-1 ml-6">
                        Attempt to continue parsing after encountering syntax errors
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* Semantic Settings */}
            {activeTab === 'semantic' && (
              <div className="space-y-6">
                <div>
                  <h3 className="text-lg font-medium text-gray-900 mb-4">
                    Semantic Analysis Configuration
                  </h3>
                  <div className="space-y-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        Maximum Resolution Depth
                      </label>
                      <input
                        type="number"
                        min="1"
                        max="50"
                        value={settings.semantic.maxResolutionDepth}
                        onChange={(e) => updateSettings('semantic', 'maxResolutionDepth', parseInt(e.target.value))}
                        className="input w-full"
                      />
                      <p className="text-xs text-gray-500 mt-1">
                        Maximum depth for symbol resolution (higher values may impact performance)
                      </p>
                    </div>

                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        Symbol Cache Size
                      </label>
                      <input
                        type="number"
                        min="100"
                        max="10000"
                        step="100"
                        value={settings.semantic.symbolCacheSize}
                        onChange={(e) => updateSettings('semantic', 'symbolCacheSize', parseInt(e.target.value))}
                        className="input w-full"
                      />
                    </div>

                    <div>
                      <label className="flex items-center">
                        <input
                          type="checkbox"
                          checked={settings.semantic.enableCrossFileAnalysis}
                          onChange={(e) => updateSettings('semantic', 'enableCrossFileAnalysis', e.target.checked)}
                          className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                        />
                        <span className="ml-2 text-sm text-gray-700">
                          Enable cross-file analysis
                        </span>
                      </label>
                      <p className="text-xs text-gray-500 mt-1 ml-6">
                        Analyze dependencies and references across multiple files
                      </p>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* Diff Engine Settings */}
            {activeTab === 'diffEngine' && (
              <div className="space-y-6">
                <div>
                  <h3 className="text-lg font-medium text-gray-900 mb-4">
                    Diff Engine Configuration
                  </h3>
                  <div className="space-y-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        Default Similarity Threshold: {settings.diffEngine.defaultSimilarityThreshold}
                      </label>
                      <input
                        type="range"
                        min="0.1"
                        max="1.0"
                        step="0.1"
                        value={settings.diffEngine.defaultSimilarityThreshold}
                        onChange={(e) => updateSettings('diffEngine', 'defaultSimilarityThreshold', parseFloat(e.target.value))}
                        className="w-full"
                      />
                      <div className="flex justify-between text-xs text-gray-500 mt-1">
                        <span>0.1 (Loose)</span>
                        <span>1.0 (Strict)</span>
                      </div>
                    </div>

                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        Maximum Tree Depth
                      </label>
                      <input
                        type="number"
                        min="5"
                        max="100"
                        value={settings.diffEngine.maxTreeDepth}
                        onChange={(e) => updateSettings('diffEngine', 'maxTreeDepth', parseInt(e.target.value))}
                        className="input w-full"
                      />
                    </div>

                    <div className="space-y-3">
                      <label className="flex items-center">
                        <input
                          type="checkbox"
                          checked={settings.diffEngine.enableRefactoringDetection}
                          onChange={(e) => updateSettings('diffEngine', 'enableRefactoringDetection', e.target.checked)}
                          className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                        />
                        <span className="ml-2 text-sm text-gray-700">
                          Enable refactoring pattern detection
                        </span>
                      </label>

                      <label className="flex items-center">
                        <input
                          type="checkbox"
                          checked={settings.diffEngine.enableCrossFileTracking}
                          onChange={(e) => updateSettings('diffEngine', 'enableCrossFileTracking', e.target.checked)}
                          className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                        />
                        <span className="ml-2 text-sm text-gray-700">
                          Enable cross-file function tracking
                        </span>
                      </label>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* Output Settings */}
            {activeTab === 'output' && (
              <div className="space-y-6">
                <div>
                  <h3 className="text-lg font-medium text-gray-900 mb-4">
                    Output Configuration
                  </h3>
                  <div className="space-y-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        Default Output Format
                      </label>
                      <select
                        value={settings.output.defaultFormat}
                        onChange={(e) => updateSettings('output', 'defaultFormat', e.target.value)}
                        className="input w-full"
                      >
                        <option value="text">Text</option>
                        <option value="json">JSON</option>
                        <option value="html">HTML</option>
                        <option value="xml">XML</option>
                        <option value="csv">CSV</option>
                        <option value="markdown">Markdown</option>
                      </select>
                    </div>

                    <div className="space-y-3">
                      <label className="flex items-center">
                        <input
                          type="checkbox"
                          checked={settings.output.enableColors}
                          onChange={(e) => updateSettings('output', 'enableColors', e.target.checked)}
                          className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                        />
                        <span className="ml-2 text-sm text-gray-700">
                          Enable colored output
                        </span>
                      </label>

                      <label className="flex items-center">
                        <input
                          type="checkbox"
                          checked={settings.output.includeTimestamps}
                          onChange={(e) => updateSettings('output', 'includeTimestamps', e.target.checked)}
                          className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                        />
                        <span className="ml-2 text-sm text-gray-700">
                          Include timestamps in output
                        </span>
                      </label>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* UI Settings */}
            {activeTab === 'ui' && (
              <div className="space-y-6">
                <div>
                  <h3 className="text-lg font-medium text-gray-900 mb-4">
                    User Interface Configuration
                  </h3>
                  <div className="space-y-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-2">
                        Theme
                      </label>
                      <select
                        value={settings.ui.theme}
                        onChange={(e) => updateSettings('ui', 'theme', e.target.value)}
                        className="input w-full"
                      >
                        <option value="light">Light</option>
                        <option value="dark">Dark</option>
                        <option value="auto">Auto (System)</option>
                      </select>
                    </div>

                    <div className="space-y-3">
                      <label className="flex items-center">
                        <input
                          type="checkbox"
                          checked={settings.ui.showLineNumbers}
                          onChange={(e) => updateSettings('ui', 'showLineNumbers', e.target.checked)}
                          className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                        />
                        <span className="ml-2 text-sm text-gray-700">
                          Show line numbers in code view
                        </span>
                      </label>

                      <label className="flex items-center">
                        <input
                          type="checkbox"
                          checked={settings.ui.enableSyntaxHighlighting}
                          onChange={(e) => updateSettings('ui', 'enableSyntaxHighlighting', e.target.checked)}
                          className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                        />
                        <span className="ml-2 text-sm text-gray-700">
                          Enable syntax highlighting
                        </span>
                      </label>

                      <label className="flex items-center">
                        <input
                          type="checkbox"
                          checked={settings.ui.autoSave}
                          onChange={(e) => updateSettings('ui', 'autoSave', e.target.checked)}
                          className="rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                        />
                        <span className="ml-2 text-sm text-gray-700">
                          Auto-save settings
                        </span>
                      </label>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* Action Buttons */}
            <div className="flex items-center justify-between pt-6 border-t border-gray-200">
              <button
                onClick={handleReset}
                className="btn-outline btn-md inline-flex items-center"
              >
                <RotateCcw className="h-4 w-4 mr-2" />
                Reset to Defaults
              </button>

              <div className="flex items-center space-x-3">
                {saveStatus === 'saved' && (
                  <div className="flex items-center text-success-600">
                    <CheckCircle className="h-4 w-4 mr-1" />
                    <span className="text-sm">Saved</span>
                  </div>
                )}
                {saveStatus === 'error' && (
                  <div className="flex items-center text-danger-600">
                    <AlertCircle className="h-4 w-4 mr-1" />
                    <span className="text-sm">Error saving</span>
                  </div>
                )}
                <button
                  onClick={handleSave}
                  disabled={saveStatus === 'saving'}
                  className={clsx(
                    'btn-md inline-flex items-center',
                    saveStatus === 'saving' ? 'btn-secondary' : 'btn-primary'
                  )}
                >
                  <Save className="h-4 w-4 mr-2" />
                  {saveStatus === 'saving' ? 'Saving...' : 'Save Settings'}
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
