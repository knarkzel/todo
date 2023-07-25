<script lang="ts">
  import { invalidateAll } from "$app/navigation";
  import type { PageData } from "./$types";

  export let data: PageData;

  async function deleteTodo(id: number) {
    await fetch(`http://0.0.0.0:8000/delete/${id}`, { method: "POST" });
    invalidateAll();
  }
</script>

<div>
  {#each data.todos as todo}
    <p>{todo.id}</p>
    <p>{todo.description}</p>
    <p>{todo.done}</p>
    <button on:click={deleteTodo(todo.id)}>Delete</button>
  {/each}
</div>

<h1>Add new todo</h1>

<form action="http://0.0.0.0:8000/create" method="POST">
  <label for="description">
    Description:
    <input name="description" type="text" placeholder="Description" />
  </label>
  
  <label for="done">
    Done:
    <input name="done" type="checkbox" value="true" />
  </label>

  <button>Create new todo</button>
</form>
