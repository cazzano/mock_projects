import { Button } from '@/components/ui/button';
import { Menu, Search, User, ShoppingCart, Heart } from 'lucide-react';

function App() {
  return (
    <div className="min-h-screen bg-white">
      {/* Navigation Bar */}
      <nav className="border-b border-t border-l border-r border-slate-800">
        <div className="max-w-7xl mx-auto px-10 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            {/* Logo */}
            <div>            
              <span className="text-2xl font-bold text-slate-900">Siddo Mosseala</span>
            </div>

            {/* Desktop Navigation */}
            <div className="hidden md:flex items-center space-x-8">
              <a href="#" className="text-slate-700 hover:text-slate-900 font-medium">Home</a>
              <a href="#" className="text-slate-700 hover:text-slate-900 font-medium">Products</a>
              <a href="#" className="text-slate-700 hover:text-slate-900 font-medium">About</a>
              <a href="#" className="text-slate-700 hover:text-slate-900 font-medium">Contact</a>
            </div>

            {/* Right Side Icons */}
            <div className="flex items-center space-x-4">
              <Button variant="ghost" size="icon">
                <Search className="h-5 w-5" />
              </Button>
              <Button variant="ghost" size="icon">
                <Heart className="h-5 w-5" />
              </Button>
              <Button variant="ghost" size="icon">
                <ShoppingCart className="h-5 w-5" />
              </Button>
              <Button variant="ghost" size="icon">
                <User className="h-5 w-5" />
              </Button>
              <Button variant="ghost" size="icon" className="md:hidden">
                <Menu className="h-5 w-5" />
              </Button>
            </div>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <section className="relative bg-gradient-to-br from-slate-50 to-slate-100">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24 md:py-32">
          <div className="text-center space-y-8">
            <h1 className="text-4xl md:text-6xl font-bold text-slate-900 tracking-tight">
              Welcome to Our Platform
            </h1>
            <p className="text-xl text-slate-600 max-w-2xl mx-auto">
              Discover amazing products and experiences crafted just for you. Start your journey today.
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Button size="lg" className="text-base px-8">
                Get Started
              </Button>
              <Button size="lg" variant="outline" className="text-base px-8">
                Learn More
              </Button>
            </div>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-slate-900 text-slate-300">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
          <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
            {/* Company Info */}
            <div className="space-y-4">
              <h3 className="text-white text-lg font-bold">Brand</h3>
              <p className="text-sm">
                Making the world a better place through innovative solutions.
              </p>
            </div>

            {/* Product Links */}
            <div className="space-y-4">
              <h4 className="text-white font-semibold">Product</h4>
              <ul className="space-y-2 text-sm">
                <li><a href="#" className="hover:text-white">Features</a></li>
                <li><a href="#" className="hover:text-white">Pricing</a></li>
                <li><a href="#" className="hover:text-white">Updates</a></li>
              </ul>
            </div>

            {/* Company Links */}
            <div className="space-y-4">
              <h4 className="text-white font-semibold">Company</h4>
              <ul className="space-y-2 text-sm">
                <li><a href="#" className="hover:text-white">About</a></li>
                <li><a href="#" className="hover:text-white">Careers</a></li>
                <li><a href="#" className="hover:text-white">Contact</a></li>
              </ul>
            </div>

            {/* Legal Links */}
            <div className="space-y-4">
              <h4 className="text-white font-semibold">Legal</h4>
              <ul className="space-y-2 text-sm">
                <li><a href="#" className="hover:text-white">Privacy</a></li>
                <li><a href="#" className="hover:text-white">Terms</a></li>
                <li><a href="#" className="hover:text-white">Cookie Policy</a></li>
              </ul>
            </div>
          </div>

          <div className="border-t border-slate-800 mt-8 pt-8 text-sm text-center">
            <p>&copy; 2025 Brand. All rights reserved.</p>
          </div>
        </div>
      </footer>
    </div>
  );
}

export default App;
