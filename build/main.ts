import ayu, { type Scheme } from "npm:ayu";

const themes = ["dark", "light", "mirage"] as const;

type Color = { _rgb: [number, number, number, number]; hex: () => string };
type Shade = { color: Color; bg: Color };

const pascalToSnake = (str: string) => {
  return str
    .split(/(?=[A-Z])/)
    .map((s) => s.toLowerCase())
    .join("_");
};

const fixShade = (shade: Shade) => {
  return [shade.color._rgb.slice(), shade.bg._rgb.slice()].map(([r, g, b, a]) =>
    [r, g, b, a * 255].map((c) => Math.round(c))
  );
};

const emitColorConst = (name: string, shade: Shade) => {
  const [fg, bg] = fixShade(shade);
  return `/// \`fg=${shade.color.hex()}\` \`bg=${shade.bg.hex()}\`
pub const ${pascalToSnake(name)}: Shade = Shade::new([${fg}], [${bg}]);`;
};

const indent = (str: string, n: number) =>
  str
    .split("\n")
    .map((line) => " ".repeat(n * 4) + line)
    .join("\n");

const buildColors = (name: string, colors: Scheme) => {
  console.log(`pub mod ${pascalToSnake(name)} {`);
  const result: Record<string, ReturnType<typeof fixShade>> = {};
  for (const key in colors) {
    const color = colors[key as keyof typeof colors];
    console.log(`    pub mod ${pascalToSnake(key)} {`);
    console.log(`        use crate::Shade;`);
    for (const subKey in color) {
      const subColor = color[subKey as keyof typeof color];
      if ("color" in subColor && "bg" in subColor) {
        console.log(indent(emitColorConst(subKey, subColor), 2));
      } else {
        console.log(`        pub mod ${pascalToSnake(subKey)} {`);
        console.log(`            use crate::Shade;`);
        for (const [subSubKey, subSubColor] of Object.entries(subColor)) {
          result[`${key}.${subKey}.${subSubKey}`] = fixShade(
            subSubColor as Shade
          );
          console.log(
            indent(emitColorConst(subSubKey, subSubColor as Shade), 3)
          );
        }
        console.log(`        }`);
      }
    }
    console.log(`    }`);
  }
  console.log(`}`);
};

for (const name of themes) {
  buildColors(name, ayu[name]);
}
