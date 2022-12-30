const COLORS = {
  "black": "#282c34",
  "white": "#abb2bf",
  "light-red": "#e06c75",
  "dark-red": "#be5046",
  "green": "#98c379",
  "light-yellow": "#e5c07b",
  "dark-yellow": "#d19a66",
  "blue": "#61afef",
  "magenta": "#c678dd",
  "cyan": "#56b6c2",
  "gutter": "#4b5263",
  "comment": "#5c6370"
}

export function setUp(monaco) {
  monaco.languages.register({id: "lux"});
  monaco.languages.setMonarchTokensProvider("lux", {
    keywords: [],
    tokenizer: {
      root: [
        [ /[a-zA-Z][\w$]*/, {
          cases: {
            "@keywords": "keyword",
            "@default": "variable"
          }
        }],
        [/\/\/.*/, "comment"]
      ]
    }
  });

  monaco.editor.defineTheme("lux-theme", {
    base: "vs-dark",
    inherit: true,
    rules: [
      { token: "keyword", foreground: COLORS["light-red"] },
      { token: "comment", foreground: COLORS["gutter"] },
      { token: "variable", foreground: COLORS["magenta"], fontStyle: "bold" },
    ],
    colors: {
      "editor.foreground": COLORS["white"],
      "editor.background": COLORS["black"],
    }
  });

  monaco.editor.setTheme("lux-theme");
}