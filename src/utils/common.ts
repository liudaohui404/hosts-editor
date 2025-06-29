function throttle(
  func: (...args: any[]) => void,
  delay: number
): (...args: any[]) => void {
  let lastFunc: ReturnType<typeof setTimeout> | null;
  let lastRan: number | null;

  return function (this: unknown, ...args: any[]) {
    let context: any = this;
    if (!lastRan) {
      func.apply(context, args);
      lastRan = Date.now();
    } else {
      if (lastFunc) clearTimeout(lastFunc);
      lastFunc = setTimeout(() => {
        if (Date.now() - (lastRan as number) >= delay) {
          func.apply(context, args);
          lastRan = Date.now();
        }
      }, delay - (Date.now() - (lastRan as number)));
    }
  };
}

function debounce(
  func: (...args: any[]) => void,
  wait: number
): (...args: any[]) => void {
  let timeout: ReturnType<typeof setTimeout> | null;

  return function (this: unknown, ...args: any[]) {
    let context: any = this;
    const later = () => {
      timeout = null;
      func.apply(context, args);
    };

    clearTimeout(timeout as ReturnType<typeof setTimeout>);
    timeout = setTimeout(later, wait);
  };
}

export { throttle, debounce };