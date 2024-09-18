export function haptic(node, duration = 50) {
    function vibrate() {
      if ('vibrate' in navigator) {
        navigator.vibrate(duration);
      }
    }
    
    node.addEventListener('click', vibrate);
    
    return {
      destroy() {
        node.removeEventListener('click', vibrate);
      }
    };
  }