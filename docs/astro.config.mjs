// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import starlightThemeRapide from "starlight-theme-rapide";
import starlightLinksValidator from "starlight-links-validator";
import starlightImageZoom from "starlight-image-zoom";

// https://astro.build/config
export default defineConfig({
  integrations: [
    starlight({
      plugins: [
        starlightThemeRapide(),
        starlightLinksValidator(),
        starlightImageZoom(),
      ],
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
                { slug: "admin/guides/import-game" },
                { slug: "admin/guides/import-version" },
              ],
            },
            {
              label: "Metadata",
              autogenerate: { directory: "admin/metadata" },
            },
            {
              label: "Authentication",
              autogenerate: { directory: "admin/authentication" },
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
  site: "https://docs-next.droposs.org/",
});
