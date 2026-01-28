// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import starlightThemeRapide from "starlight-theme-rapide";

// https://astro.build/config
export default defineConfig({
  integrations: [
    starlight({
      plugins: [starlightThemeRapide()],
      title: "Drop OSS",
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/Drop-OSS/",
        },
      ],
      sidebar: [
        {
          label: "Admin",
          items: [
            { slug: "admin/quickstart" },
            {
              label: "Guides",
              items: [
                { slug: "admin/guides/exposing" },
                { slug: "admin/guides/creating-library" },
              ],
            },
            {
              label: "Metadata",
              autogenerate: { directory: "admin/metadata" },
            },
          ],
        },
        {
          label: "Reference",
          autogenerate: { directory: "reference" },
        },
      ],
      customCss: ["./src/styles/drop.css"],
    }),
  ],
});
