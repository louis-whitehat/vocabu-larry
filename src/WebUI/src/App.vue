<template>
  <div class="app-shell">
    <header class="app-toolbar">
      <router-link :to="{ name: 'login' }" class="brand-mark">Vocabu-Larry</router-link>

      <nav class="toolbar-actions" aria-label="Global navigation">
        <router-link
          v-if="route.name !== 'login'"
          :to="{ name: 'login' }"
          class="toolbar-action"
          aria-label="Go home"
          title="Home"
        >
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <path d="M3 10.5 12 3l9 7.5M6.75 8.75V21h10.5V8.75M10 21v-6h4v6" />
          </svg>
        </router-link>

        <router-link
          v-if="route.name !== 'logs'"
          :to="{ name: 'logs' }"
          class="toolbar-action"
          aria-label="Open logs"
          title="Logs"
        >
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <path
              d="M6 3h9l5 5v13a1 1 0 0 1-1 1H6a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2Zm8 1.5V9h4.5M8 13h8M8 17h8M8 9h3"
            />
          </svg>
        </router-link>
      </nav>
    </header>

    <main class="page-frame">
      <RouterView />
    </main>
  </div>
</template>

<script setup>
  import { useRoute } from 'vue-router'

  const route = useRoute()
</script>

<style>
  :root {
    --app-blue-strong: #173a63;
    --app-blue: #245d93;
    --app-blue-soft: #4a79b6;
    --app-blue-muted: #395b84;
    --app-blue-border: rgba(54, 105, 170, 0.14);
    --app-blue-panel: rgba(242, 248, 255, 0.96);
    --app-blue-shadow: rgba(27, 73, 125, 0.12);
  }

  body {
    margin: 0;
    min-width: 320px;
    background:
      radial-gradient(circle at top left, rgba(163, 210, 255, 0.28), transparent 32%),
      linear-gradient(180deg, #f7fbff, #eef6ff 65%, #f8fbff);
  }

  #app {
    min-height: 100vh;
  }

  .app-shell {
    max-width: 960px;
    margin: 0 auto;
    padding: 18px 18px 40px;
  }

  .app-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
    margin-bottom: 28px;
  }

  .brand-mark {
    color: var(--app-blue-strong);
    font-size: 1.95rem;
    font-weight: 700;
    letter-spacing: -0.03em;
    text-decoration: none;
  }

  .brand-mark:hover {
    color: var(--app-blue);
  }

  .toolbar-actions {
    display: flex;
    gap: 12px;
  }

  .toolbar-action {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    border: 1px solid rgba(54, 105, 170, 0.2);
    border-radius: 999px;
    background: var(--app-blue-panel);
    color: #2d5f99;
    box-shadow: 0 10px 24px rgba(27, 73, 125, 0.1);
    transition:
      transform 140ms ease,
      box-shadow 140ms ease,
      border-color 140ms ease;
  }

  .toolbar-action:hover {
    transform: translateY(-1px);
    border-color: rgba(54, 105, 170, 0.38);
    box-shadow: 0 14px 30px rgba(27, 73, 125, 0.16);
  }

  .toolbar-action svg {
    width: 20px;
    height: 20px;
    fill: none;
    stroke: currentColor;
    stroke-linecap: round;
    stroke-linejoin: round;
    stroke-width: 1.8;
  }

  .page-frame {
    text-align: center;
  }

  .page-shell {
    max-width: 860px;
    margin: 0 auto;
  }

  .panel-card {
    padding: 28px;
    border: 1px solid var(--app-blue-border);
    border-radius: 28px;
    background:
      radial-gradient(circle at top right, rgba(163, 210, 255, 0.52), transparent 28%),
      linear-gradient(180deg, rgba(247, 251, 255, 0.98), rgba(238, 246, 255, 0.98));
    box-shadow: 0 24px 60px var(--app-blue-shadow);
    text-align: left;
  }

  .page-title {
    margin: 0 0 10px;
    color: var(--app-blue-strong);
  }

  .page-copy {
    margin: 0 0 24px;
    max-width: 44rem;
    color: var(--app-blue-muted);
    font-size: 1.02rem;
  }

  .form-grid {
    display: grid;
    grid-template-columns: minmax(150px, 220px) minmax(0, 1fr);
    align-items: center;
    gap: 14px 20px;
  }

  .field-label {
    font-weight: 700;
    color: #244970;
    text-align: left;
  }

  .actions-row {
    margin-top: 24px;
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    justify-content: flex-start;
  }

  .secondary-action {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 10px 16px;
    border-radius: 999px;
    background: var(--app-blue);
    color: #f4f9ff;
    text-decoration: none;
    font-weight: 700;
  }

  .secondary-action:hover {
    background: #1d4f7d;
    color: #f4f9ff;
  }

  .muted-note {
    color: var(--app-blue-muted);
  }

  @media (max-width: 640px) {
    .app-shell {
      padding: 12px 12px 28px;
    }

    .app-toolbar {
      margin-bottom: 20px;
    }

    .brand-mark {
      font-size: 1.7rem;
    }

    .panel-card {
      padding: 22px 18px;
      border-radius: 22px;
    }

    .form-grid {
      grid-template-columns: 1fr;
      gap: 8px;
    }

    .actions-row {
      margin-top: 20px;
    }
  }
</style>
