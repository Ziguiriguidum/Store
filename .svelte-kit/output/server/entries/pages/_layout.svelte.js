import { c as create_ssr_component } from "../../chunks/index.js";
const styles = "";
const Layout = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  return `<div class="${"app"}">${slots.default ? slots.default({}) : ``}</div>`;
});
export {
  Layout as default
};
