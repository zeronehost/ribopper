export const debounce = (func: () => any, delay: number = 300) => {
  let timer: number | undefined;
  return function (this: any, ...args: any) {
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => {
      func.apply(this, args);
    }, delay);
  };
};

