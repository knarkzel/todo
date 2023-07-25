<script lang="ts">
  import type { PageData } from "./$types";

  // Get todos from page load
  export let data: PageData;
  let todos = data.todos;

  // Delete todo
  async function deleteTodo(id: number) {
    await fetch(`http://0.0.0.0:8000/delete/${id}`, { method: "POST" });
    todos = todos.filter((todo) => todo.id !== id);
  }

  // Update todo
  async function updateTodo(todo: Todo) {
    await fetch(`http://0.0.0.0:8000/update?id=${todo.id}&description=${todo.description}&done=${todo.done}`);
  }
</script>

<div class="container mx-auto mt-16">
  <h1 class="h1 text-center">Todos</h1>

  <div class="max-w-screen-md mx-auto">
    <form action="http://0.0.0.0:8000/create" method="POST">
      <input
        class="input p-4 my-8"
        name="description"
        type="text"
        placeholder="What needs to be done?"
        autocomplete="off"
      />
    </form>

    <div class="space-y-4">
      {#each todos as todo}
        <div class="flex items-center justify-between p-4 bg-surface-800 rounded-lg gap-4">
          <input
            class="checkbox"
            type="checkbox"
            bind:checked={todo.done}
            on:change={updateTodo(todo)}
          />
          <input class="input" type="text" bind:value={todo.description} disabled={todo.done} />

          <div class="flex gap-2">
            <button class="btn variant-filled-secondary" on:click={updateTodo(todo)}>Update</button>
            <button class="btn variant-filled-primary" on:click={deleteTodo(todo.id)}>Delete</button
            >
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
