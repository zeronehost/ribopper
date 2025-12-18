import { defineConfig } from "tsdown";
import { fileURLToPath, URL } from "node:url";

export default defineConfig({
  entry: ["src/index.ts"],
  alias: {
    "@": fileURLToPath(new URL("./src", import.meta.url)),
  },
  exports: true,
  format: ["esm"]
})