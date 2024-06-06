import { confirm } from '@tauri-apps/api/dialog'
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import {
  Button,
  PaginatedList,
  PaginationControls,
  ProgressBar,
  Table,
} from 'shelf-ui'
import { Show, onMount } from 'solid-js'

function App() {
  onMount(async () => {
    const { shouldUpdate, manifest } = await checkUpdate()

    if (!shouldUpdate) {
      return
    }

    const acceptUpdate = await confirm(
      `Nova versão disponível: ${manifest?.version}.\n\nDeseja atualizar o aplicativo?`,
      {
        cancelLabel: 'Não',
        okLabel: 'Sim',
        title: 'Atualização',
        type: 'info',
      },
    )

    if (!acceptUpdate) {
      return
    }

    await installUpdate()
  })

  return (
    <div class="grid h-screen w-screen grid-cols-[minmax(40ch,_1fr)_3fr] gap-2 overflow-hidden">
      <section class="flex h-full flex-col gap-4 overflow-auto bg-neutral-200 p-4 dark:bg-neutral-900">
        <h1 class="text-xl font-bold">app_name</h1>

        <PaginatedList
          class="divider-neutral-600 mb-auto min-h-[10em] flex-grow divide-y-2 overflow-auto rounded bg-neutral-300 p-2 dark:bg-neutral-700"
          pageSize={100}
          items={[]}
          render={x => x}
        />

        <Show when={true}>
          <ProgressBar displayMax={0} displayValue={0} value={0} max={0} />
        </Show>

        <Button disabled={false} onClick={() => undefined}>
          Importar arquivo
        </Button>

        <Button variant="success" disabled={true} onClick={() => undefined}>
          Pesquisar
        </Button>
      </section>

      <section class="flex-col items-center overflow-auto em:gap-2">
        <PaginationControls
          page={0}
          onPageChange={async () => undefined}
          numberOfPages={1}
        />

        <Table
          class="[&_td]:align-baseline [&_td_>_div]:line-clamp-5 [&_td_>_p]:w-max [&_td_>_p]:max-w-[75ch]"
          columns={[]}
          data={[]}
        />
      </section>

      <Show when={false}>
        <Button
          onClick={() => undefined}
          variant="primary"
          class="fixed bottom-5 right-5"
        >
          Exportar
        </Button>
      </Show>
    </div>
  )
}

export default App
