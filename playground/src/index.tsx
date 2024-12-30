import { createRoot } from 'react-dom/client';
import Router from './router';
import './index.css';

const container = document.querySelector('#root');
if (container) {
  const root = createRoot(container);
  root.render(<Router />);
}
