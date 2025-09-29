import { MainLayout } from '@/components/layout/MainLayout';
import { DiffComparison } from '@/components/diff/DiffComparison';

export default function HomePage() {
  return (
    <MainLayout>
      <div className="flex-1 flex flex-col">
        <header className="border-b bg-card px-6 py-4">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-foreground">Smart Diff</h1>
              <p className="text-sm text-muted-foreground">
                Advanced code comparison with graph-based function matching
              </p>
            </div>
            <div className="flex items-center gap-4">
              <button className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors">
                New Comparison
              </button>
            </div>
          </div>
        </header>
        
        <main className="flex-1 overflow-hidden">
          <DiffComparison />
        </main>
      </div>
    </MainLayout>
  );
}
