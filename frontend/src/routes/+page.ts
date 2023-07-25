import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  return {
    todos: (await fetch("http://0.0.0.0:8000").then((data) => data.json())) as Todo[]
  };
};
