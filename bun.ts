console.log(Bun.color("#ff595e", "{rgba}"));

let color = Bun.color("#ff595e", "{rgba}");
color.a = 0.5;
console.log(Bun.color(color, "css"));