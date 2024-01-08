export const abbreviateNumber = (num: number) => {
  if (num < 1000) {
    return num;
  }
  const si = [
    { v: 1e3, s: "K" },
    { v: 1e6, s: "M" },
  ];
  let index;
  for (index = si.length - 1; index > 0; index--) {
    if (num >= si[index].v) {
      break;
    }
  }
  return (
    (num / si[index].v).toFixed(2).replace(/\.0+$|(\.[0-9]*[1-9])0+$/, "$1") +
    si[index].s
  );
};
