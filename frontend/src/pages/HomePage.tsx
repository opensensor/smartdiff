import React from 'react';
import { Link } from 'react-router-dom';
import { 
  FileCompare, 
  Search, 
  GitBranch, 
  Zap, 
  Shield, 
  Code2,
  ArrowRight,
  CheckCircle
} from 'lucide-react';

export const HomePage: React.FC = () => {
  const features = [
    {
      icon: FileCompare,
      title: 'Structural Comparison',
      description: 'Compare code at the AST level, not just line-by-line. Understand the true structural differences between code versions.',
    },
    {
      icon: GitBranch,
      title: 'Refactoring Detection',
      description: 'Automatically detect common refactoring patterns like method extraction, renaming, and code movement.',
    },
    {
      icon: Search,
      title: 'Function Matching',
      description: 'Advanced algorithm to match functions across files, even when moved or renamed, with similarity scoring.',
    },
    {
      icon: Zap,
      title: 'Multi-Language Support',
      description: 'Support for Java, Python, JavaScript, C++, and C with consistent analysis across all languages.',
    },
    {
      icon: Shield,
      title: 'Semantic Analysis',
      description: 'Deep semantic understanding with symbol resolution, type analysis, and dependency tracking.',
    },
    {
      icon: Code2,
      title: 'Multiple Views',
      description: 'Side-by-side, unified, and function-centric views to visualize changes in the way that works best for you.',
    },
  ];

  const useCases = [
    'Code review and pull request analysis',
    'Refactoring impact assessment',
    'Legacy code modernization tracking',
    'API evolution analysis',
    'Code quality improvement monitoring',
    'Educational code comparison',
  ];

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="bg-gradient-to-br from-primary-50 to-blue-50 py-20">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center">
            <h1 className="text-4xl md:text-6xl font-bold text-gray-900 mb-6">
              Smart Code
              <span className="text-primary-600"> Diffing</span>
            </h1>
            <p className="text-xl md:text-2xl text-gray-600 mb-8 max-w-3xl mx-auto">
              Next-generation code comparison tool that understands your code's structure, 
              not just its text. Detect refactoring patterns, track function movements, 
              and analyze semantic changes with precision.
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Link
                to="/compare"
                className="btn-primary btn-lg inline-flex items-center"
              >
                Start Comparing
                <ArrowRight className="ml-2 h-5 w-5" />
              </Link>
              <Link
                to="/analyze"
                className="btn-outline btn-lg inline-flex items-center"
              >
                Analyze Code
                <Search className="ml-2 h-5 w-5" />
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20 bg-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-16">
            <h2 className="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
              Powerful Features
            </h2>
            <p className="text-xl text-gray-600 max-w-2xl mx-auto">
              Built with advanced algorithms and semantic analysis to provide 
              the most accurate code comparison experience.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            {features.map((feature, index) => {
              const Icon = feature.icon;
              return (
                <div
                  key={index}
                  className="card p-6 hover:shadow-md transition-shadow"
                >
                  <div className="flex items-center mb-4">
                    <div className="p-2 bg-primary-100 rounded-lg">
                      <Icon className="h-6 w-6 text-primary-600" />
                    </div>
                    <h3 className="text-lg font-semibold text-gray-900 ml-3">
                      {feature.title}
                    </h3>
                  </div>
                  <p className="text-gray-600">
                    {feature.description}
                  </p>
                </div>
              );
            })}
          </div>
        </div>
      </section>

      {/* Use Cases Section */}
      <section className="py-20 bg-gray-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
            <div>
              <h2 className="text-3xl md:text-4xl font-bold text-gray-900 mb-6">
                Perfect for Every
                <span className="text-primary-600"> Use Case</span>
              </h2>
              <p className="text-lg text-gray-600 mb-8">
                Whether you're reviewing code, tracking refactoring progress, 
                or analyzing API changes, Smart Code Diff provides the insights 
                you need to make informed decisions.
              </p>
              <div className="space-y-4">
                {useCases.map((useCase, index) => (
                  <div key={index} className="flex items-center">
                    <CheckCircle className="h-5 w-5 text-success-500 mr-3 flex-shrink-0" />
                    <span className="text-gray-700">{useCase}</span>
                  </div>
                ))}
              </div>
            </div>
            <div className="relative">
              <div className="card p-6 bg-gradient-to-br from-white to-gray-50">
                <div className="space-y-4">
                  <div className="flex items-center justify-between">
                    <span className="text-sm font-medium text-gray-500">Similarity Score</span>
                    <span className="text-2xl font-bold text-success-600">87.3%</span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div className="bg-success-500 h-2 rounded-full" style={{ width: '87.3%' }}></div>
                  </div>
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span className="text-gray-600">Functions matched:</span>
                      <span className="font-medium">24/27</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600">Refactoring patterns:</span>
                      <span className="font-medium">3 detected</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-600">Cross-file moves:</span>
                      <span className="font-medium">2 found</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20 bg-primary-600">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-3xl md:text-4xl font-bold text-white mb-6">
            Ready to Transform Your Code Analysis?
          </h2>
          <p className="text-xl text-primary-100 mb-8 max-w-2xl mx-auto">
            Start comparing your code with advanced structural analysis today. 
            No setup required, just upload and compare.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link
              to="/compare"
              className="btn-lg bg-white text-primary-600 hover:bg-gray-50 inline-flex items-center"
            >
              Get Started Now
              <ArrowRight className="ml-2 h-5 w-5" />
            </Link>
            <a
              href="/docs"
              className="btn-lg border-2 border-white text-white hover:bg-white hover:text-primary-600 inline-flex items-center transition-colors"
            >
              View Documentation
            </a>
          </div>
        </div>
      </section>
    </div>
  );
};
