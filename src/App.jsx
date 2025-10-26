import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Heart, Star, Zap } from 'lucide-react';

function App() {
  const [count, setCount] = useState(0);

  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 p-8">
      <div className="max-w-4xl mx-auto space-y-6">
        <div className="text-center space-y-2">
          <h1 className="text-4xl font-bold text-slate-900">
            React + Vite + shadcn/ui
          </h1>
          <p className="text-slate-600">With pnpm! 🚀</p>
        </div>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Zap className="w-5 h-5 text-yellow-500" />
              Counter Example
            </CardTitle>
            <CardDescription>Click the button to increment</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="text-center">
              <div className="text-5xl font-bold text-slate-900 mb-4">{count}</div>
              <Button onClick={() => setCount(count + 1)}>
                <Star className="w-4 h-4 mr-2" />
                Increment
              </Button>
            </div>
          </CardContent>
        </Card>

        <div className="flex gap-2 justify-center">
          <Badge>React</Badge>
          <Badge variant="secondary">Vite</Badge>
          <Badge variant="outline">shadcn/ui</Badge>
        </div>
      </div>
    </div>
  );
}

export default App;
