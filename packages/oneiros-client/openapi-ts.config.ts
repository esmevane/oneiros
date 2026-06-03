import { defineConfig } from "@hey-api/openapi-ts";

export default defineConfig({
  input: "./schema.json",
  output: "src/generated",
  plugins: ["@hey-api/client-fetch"],
});
