import { instantiate } from "./lib/rs_lib.generated.js";

const { generate } = await instantiate();

const bytes = generate(
  Deno.args[0],
);

try {
  await Deno.writeFile("qr.png", bytes);
} catch (e) {
  console.error(e);
}
