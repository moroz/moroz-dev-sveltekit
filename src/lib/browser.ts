export const isSafari = () => {
  if (typeof document === "undefined" || typeof window === "undefined")
    return false;

  return (
    "WebkitAppearance" in document.documentElement.style &&
    !("chrome" in window)
  );
};

export const isTouchDevice = () => {
  if (typeof document === "undefined" || typeof window === "undefined")
    return false;
  return "ontouchstart" in window || "onmsgesturechange" in window;
};

export const isDesktop = () => {
  if (typeof document === "undefined" || typeof window === "undefined")
    return false;
  return window.screenX != 0 && !isTouchDevice() ? true : false;
};
