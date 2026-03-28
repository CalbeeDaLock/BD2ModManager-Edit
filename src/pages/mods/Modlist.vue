<script setup lang="ts">
import { h, ref, computed, onActivated, onDeactivated, nextTick, onMounted, watch } from 'vue'
import { useVirtualizer } from '@tanstack/vue-virtual'
import { BD2ModExtended } from '../../stores/mods'
import {
    FlexRender,
    useVueTable,
    getCoreRowModel,
    createColumnHelper,
    RowSelectionState,
    SortingState,
    getSortedRowModel
} from '@tanstack/vue-table'
import Checkbox from '../../components/common/Checkbox.vue'
import { usePreferencesStore } from '../../stores/preferences'
import { AlertTriangle, ArrowDownWideNarrow, ArrowUpNarrowWide, TriangleAlert } from 'lucide-vue-next'
import ErrorTag from './ErrorTag.vue'
import type { ContextMenuItem } from '../../components/common/ContextMenu.vue'
import ContextMenu from '../../components/common/ContextMenu.vue'
import { useI18n } from 'vue-i18n'
import { useLocalStorage, useStorageAsync } from '@vueuse/core'
import { useRouter } from 'vue-router'
import Image from '../../components/common/Image.vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { useAppDir } from '../../composables/useAppDir'

const baseDir = useAppDir()

const router = useRouter()

const preferencesStore = usePreferencesStore()

const columnSizes = useLocalStorage("modlist-column-sizes", {} as Record<string, number>)

const allColumns = computed(() => [
    'name',
    'modType',
    'character',
    'author'
])
const columnVisibility = computed(() => {
    return allColumns.value.reduce((acc, column) => {
        acc[column] = preferencesStore.visibleModListColumns.includes(column)
        return acc
    }, {} as Record<string, boolean>)
})

const props = withDefaults(defineProps<{
    mods: BD2ModExtended[]
}>(), {
    mods: () => []
})

const emit = defineEmits([
    "enable-mods",
    "disable-mods",
    "refresh-mods",
    "change-mod-author",
    "rename-mod",
    "delete-mods",
    "preview-mod",
    "open-mod-folder",
    "show-mod-conflicts"
])

const columnHelper = createColumnHelper<BD2ModExtended>()

function getErrorDescription(error: string) {
    const errors: Record<string, string> = {
        "MissingTextures": "modErrors.missingTextures",
        "MissingModfile": "modErrors.missingModfile",
        "IsNotExtracted": "modErrors.isNotExtracted",
        "ArchiveNotExtracted": "modErrors.archiveNotExtracted",
        "ShouldBeInFolder": "modErrors.shouldBeInFolder",
        "MissingAtlasFile": "modErrors.missingAtlasFile",
    }

    return t(errors[error] || 'modErrors.noDescription', { error })
}

function getTypeClass(type: string) {
    if (!preferencesStore.enableModTypeColors) return ''
    
    return {
        'Cutscene': 'font-medium text-cutscene',
        'Standing': 'font-medium text-standing',
        'Scene': 'font-medium text-scene',
        'Dating': 'font-medium text-dating',
        'NPC': 'font-medium text-npc',
        'Minigame': 'font-medium text-minigame'
    }[type] || ''
}

const columns = [
    columnHelper.accessor('name', {
        cell: info => {
            const mod = info.row.original
            return h('div', { class: 'flex items-center gap-2 overflow-hidden' }, [
                mod.errors.length > 0 ? h('span', { class: 'select-none rounded-lg text-sm font-mono text-red-200' },
                    h(AlertTriangle, { class: 'text-red-400 w-[1.25rem] h-[1.25rem]' })
                ) :
                    h(Checkbox, {
                        modelValue: mod.enabled,
                        'onUpdate:modelValue': (val: boolean) => {
                            emit(val ? 'enable-mods' : 'disable-mods', [mod])
                            // mod.enabled = false
                        },
                        onClick: (e: Event) => e.stopPropagation()
                    }),
                h('span', { class: 'truncate text-primary flex-1' }, preferencesStore.modNameDisplay === "short"? mod.displayName: mod.name),
                mod.conflictingMods && mod.conflictingMods.length > 0 ? h('button', {
                    onClick: (e: Event) => {
                        emit('show-mod-conflicts', mod)
                        e.stopPropagation()
                    },

                    class: 'conflicts-indicator cursor-pointer text-xs group flex items-center gap-1 py-1 hover:bg-yellow-500 transition-colors bg-yellow-600 font-bold font-mono box-border px-1.5 rounded-sm h-6'
                }, [
                    h(TriangleAlert, { class: 'w-4 h-4 text-primary transition-colors' }),
                    h('span', { class: 'text-primary font-bold text-xs' }, mod.conflictingMods.length)
                ]) : null,
                mod.errors.length > 0? h('div', { class: 'flex gap-2 flex-wrap' },
                    mod.errors.map((error, index) =>
                        h(ErrorTag, {
                            key: index,
                            label: error,
                            description: getErrorDescription(error)
                        })
                    )
                ) : null
            ])
        },
        header: () => h('span', { class: 'flex text-primary items-center' }, t('modsTab.modlist.header.modName')),
        size: 300
    }),
    columnHelper.accessor('modType', {
        cell: info => {
            let modType: string = info.getValue()?.type
            if (!modType) return null
            return h('div', {
                class: ['flex items-center gap-1.5', preferencesStore.modTypeDisplay === 'iconOnly' ? 'justify-center' : '']
            }, [
                preferencesStore.modTypeDisplay === 'iconOnly' || preferencesStore.modTypeDisplay === 'full' ? h(Image, {
                    class: "w-[1.25rem] h-[1.25rem] object-contain",
                    src: `/icons/${modType.toLowerCase()}.png`
                }) : null,
                preferencesStore.modTypeDisplay === 'labelOnly' || preferencesStore.modTypeDisplay === 'full' ? h('span', {
                    class: ['font-base', getTypeClass(modType)]
                }, modType ?? null) : null
            ])
        },
        header: () => h('span', { class: 'flex text-primary' }, t('modsTab.modlist.header.modType')),
        // accessorFn: row => row.modType,
        sortUndefined: "last",
        sortDescFirst: false,
        sortingFn: (rowA, rowB, columnId) => {
            const modTypeA = (rowA.getValue(columnId) as any)?.type ?? undefined
            const modTypeB = (rowB.getValue(columnId) as any)?.type ?? undefined

            // empty goes last
            // if (modTypeA === '' && modTypeB === '') return 0

            if (modTypeA === modTypeB) return 0
            if (!modTypeA) return 1
            if (!modTypeB) return -1



            return modTypeA.localeCompare(modTypeB, undefined, { sensitivity: 'base' })
        },
        size: 150 // asdhasdhasgasghdhasdhasdhhasdashghsasasdhagsdghjasdghjasghjasdghhgasdghasdghj
    }),
    columnHelper.accessor('character', {
        // accessorFn: row => row.character == null ? undefined : row.character,
        cell: info => {
            const char = info.getValue()
            if (char) {

                return h(
                    "div",
                    {
                        class: ['flex items-center gap-1 overflow-hidden', preferencesStore.characterDisplay === 'iconOnly' ? 'justify-center' : '']
                    },
                    [
                        preferencesStore.characterDisplay === 'iconOnly' || preferencesStore.characterDisplay === 'full' ? h(Image, {
                            src: `/characters/heads/${char.id}.png`,
                            loading: 'lazy',
                            class: 'w-[2rem] h-[2rem] object-cover scale-150 cursor-pointer hover:scale-165 transition-transform',
                            fallbackSources: [
                                convertFileSrc(`${baseDir.value}/assets/heads/${char.id}.png`),
                                '/characters/heads/050001.png'
                            ],
                            onClick: (event: Event) => {
                                event.stopPropagation()
                                // redirects to character page
                                router.push({ name: 'characters', query: { characterId: char.id } })
                            }
                        }) : null,
                        preferencesStore.characterDisplay === 'nameOnly' || preferencesStore.characterDisplay === 'full' ? h('span', { class: 'truncate text-sm text-primary' }, `${char.character} - ${char.costume}`) : null
                    ]
                )
            }
        },
        header: () => h('span', { class: 'flex text-primary' }, t('modsTab.modlist.header.character')),
        sortingFn: (rowA, rowB, columnId) => {
            const charA: any = rowA.getValue(columnId);
            const charB: any = rowB.getValue(columnId);

            return (
                charA?.character.localeCompare(charB?.character, undefined, { sensitivity: 'base' }) ||
                charA?.costume.localeCompare(charB?.costume, undefined, { sensitivity: 'base' })
            );
        },
        sortDescFirst: false,
        sortUndefined: "last",
        size: 200
    }),
    columnHelper.accessor('author', {
        cell: info => {
            return h('span', { class: 'truncate text-sm text-primary' }, info.getValue())
        },
        header: () => h('span', { class: 'flex text-primary' }, t('modsTab.modlist.header.author')),
        size: 200
    }),
]

const rowSelection = ref<RowSelectionState>({})
const anchorRowIndex = ref<number | null>(null)
const metaKeySelection = ref(true)

const showDropdown = ref(false)
const x = ref(0)
const y = ref(0)

function handleRowClick(e: MouseEvent, rowIndex: number) {
    const row = rows.value[rowIndex]
    const isSelected = row.getIsSelected()

    if (e.shiftKey && anchorRowIndex.value !== null) {
        const start = Math.min(anchorRowIndex.value, rowIndex)
        const end = Math.max(anchorRowIndex.value, rowIndex)

        table.resetRowSelection()
        for (let i = start; i <= end; i++) {
            rows.value[i].toggleSelected(true)
        }
        return
    }

    if (e.ctrlKey || e.metaKey) {
        anchorRowIndex.value = rowIndex
        row.toggleSelected()
        return
    }

    anchorRowIndex.value = rowIndex

    if (metaKeySelection.value) {
        table.resetRowSelection()
        row.toggleSelected(true)
    } else {
        if (isSelected) {
            row.toggleSelected(false)
        } else {
            table.resetRowSelection()
            row.toggleSelected(true)
        }
    }
}
const sorting = useStorageAsync<SortingState>("modlist-sorting", [
    { id: 'name', desc: false }
])

const table = useVueTable({
    get data() {
        return props.mods
    },
    columns,
    getCoreRowModel: getCoreRowModel(),
    getRowId: row => row.name,
    enableColumnResizing: true,
    columnResizeMode: 'onEnd',
    columnResizeDirection: 'ltr',
    enableSortingRemoval: false,
    enableRowSelection: true,
    enableMultiRowSelection: true,
    enableHiding: true,
    state: {
        get rowSelection() {
            return rowSelection.value
        },
        get sorting() {
            return sorting.value
        },
        get columnSizing() {
            return columnSizes.value
        },
        get columnVisibility() {
            return columnVisibility.value
        }
    },
    onRowSelectionChange: updateOrValue => {
        rowSelection.value =
            typeof updateOrValue === 'function'
                ? updateOrValue(rowSelection.value)
                : updateOrValue
    },
    onSortingChange: updaterOrValue => {
        sorting.value =
            typeof updaterOrValue === 'function'
                ? updaterOrValue(sorting.value)
                : updaterOrValue
    },
    getSortedRowModel: getSortedRowModel(),
    onColumnSizingChange: updaterOrValue => {
        columnSizes.value =
            typeof updaterOrValue === 'function'
                ? updaterOrValue(columnSizes.value)
                : updaterOrValue
    },
})

const rows = computed(() => table.getRowModel().rows)

const selectedMods = computed(() => {
    return rows.value
        .filter(row => row.getIsSelected())
        .map(row => row.original)
})

const parentRef = ref<HTMLElement | null>(null)

const rowVirtualizerOptions = computed(() => ({
    count: rows.value.length,
    getScrollElement: () => parentRef.value,
    estimateSize: () => 54,
    overscan: 10,
}))

const rowVirtualizer = useVirtualizer(rowVirtualizerOptions)

const virtualRows = computed(() => rowVirtualizer.value.getVirtualItems())
const totalSize = computed(() => rowVirtualizer.value.getTotalSize())

const gridTemplateColumns = computed(() => {
    const headers = table.getFlatHeaders()
    const columns = headers.map((header, index) => {
        if (index === headers.length - 1) {
            return 'minmax(200px, 1fr)'
        }
        return `${header.getSize()}px`
    })
    return columns.join(' ')
})

const scrollTop = ref(0)

onDeactivated(() => {
    if (parentRef.value) scrollTop.value = parentRef.value.scrollTop
})

onActivated(() => {
    if (parentRef.value) parentRef.value.scrollTop = scrollTop.value
})

const { t } = useI18n()

const contextMenuItems = computed<ContextMenuItem[]>(() => {
    const hasSelection = selectedMods.value.length > 0
    const isSingleSelection = selectedMods.value.length === 1

    return [
        {
            label: t('modsTab.modlist.contextMenu.refreshMods'),
            key: 'refresh'
        } as ContextMenuItem,
        {
            type: 'divider' as const,
            key: 'd1'
        } as ContextMenuItem,
        {
            label: t('modsTab.modlist.contextMenu.selectMods'),
            key: 'select-mods',
            children: [
                { label: t('modsTab.modlist.contextMenu.selectAllMods'), key: 'select-all' },
                { label: t('modsTab.modlist.contextMenu.selectEnabledMods'), key: 'select-enabled' },
                { label: t('modsTab.modlist.contextMenu.selectDisabledMods'), key: 'select-disabled' },
                { type: 'divider' as const, key: 'd-select' },
                { label: t('modsTab.modlist.contextMenu.deselectMods'), key: 'deselect-all' },
            ]
        } as ContextMenuItem,
        {
            type: 'divider' as const,
            key: 'd2',
            show: hasSelection
        } as ContextMenuItem,
        {
            label: isSingleSelection ? t('modsTab.modlist.contextMenu.enableMod') : t('modsTab.modlist.contextMenu.enableSelectedMods'),
            key: 'enable',
            show: hasSelection && selectedMods.value.some(mod => !mod.enabled && mod.errors.length === 0)
        } as ContextMenuItem,
        {
            label: isSingleSelection ? t('modsTab.modlist.contextMenu.disableMod') : t('modsTab.modlist.contextMenu.disableSelectedMods'),
            key: 'disable',
            show: hasSelection && selectedMods.value.some(mod => mod.enabled && mod.errors.length === 0)
        } as ContextMenuItem,
        {
            label: isSingleSelection ? t('modsTab.modlist.contextMenu.changeModAuthor') : t('modsTab.modlist.contextMenu.changeSelectedModsAuthor'),
            key: 'change-author',
            show: hasSelection
        } as ContextMenuItem,
        // {
        //     label: t('modsTab.modlist.contextMenu.editModFile'),
        //     key: 'edit-modfile',
        //     show: isSingleSelection
        // },
        {
            type: 'divider' as const,
            key: 'd3',
            show: hasSelection
        } as ContextMenuItem,
        {
            label: t('modsTab.modlist.contextMenu.renameMod'),
            key: 'rename',
            show: isSingleSelection
        } as ContextMenuItem,
        {
            label: isSingleSelection ? t('modsTab.modlist.contextMenu.deleteMod') : t('modsTab.modlist.contextMenu.deleteSelectedMods'),
            key: 'delete',
            show: hasSelection
        } as ContextMenuItem,
        {
            type: 'divider' as const,
            key: 'd4',
            show: isSingleSelection
        } as ContextMenuItem,
        {
            label: t('modsTab.modlist.contextMenu.openModFolder'),
            key: 'open-folder',
            show: isSingleSelection
        } as ContextMenuItem,
        {
            label: t('modsTab.modlist.contextMenu.previewMod'),
            key: 'preview',
            show: isSingleSelection
        } as ContextMenuItem,
    ].filter(item => item.show !== false)
})

function handleContextMenu(event: MouseEvent, rowIndex: number) {
    event.preventDefault()

    const row = rows.value[rowIndex]
    const isSelected = row.getIsSelected()

    if (!isSelected) {
        table.resetRowSelection()
        row.toggleSelected(true)
        anchorRowIndex.value = rowIndex
    }

    showDropdown.value = false
    nextTick().then(() => {
        showDropdown.value = true
        x.value = event.clientX
        y.value = event.clientY
    })
}

function handleSelect(key: string) {
    showDropdown.value = false

    const actions: Record<string, () => void> = {
        'refresh': () => {
            emit('refresh-mods')
        },
        'select-all': () => {
            rows.value.forEach(row => row.toggleSelected(true))
        },
        'select-enabled': () => {
            table.resetRowSelection()
            rows.value
                .filter(row => row.original.enabled)
                .filter(row => row.original.errors.length === 0)
                .forEach(row => row.toggleSelected(true))
        },
        'select-disabled': () => {
            table.resetRowSelection()
            rows.value
                .filter(row => !row.original.enabled)
                .filter(row => row.original.errors.length === 0)
                .forEach(row => row.toggleSelected(true))
        },
        'deselect-all': () => table.resetRowSelection(),
        'enable': () => emit('enable-mods', selectedMods.value),
        'disable': () => emit('disable-mods', selectedMods.value),
        'change-author': () => {
            emit('change-mod-author', selectedMods.value)
        },
        // 'edit-modfile': () => {
        //     emit('edit-modfile', selectedMods.value[0])
        // },
        'rename': () => {
            emit('rename-mod', selectedMods.value[0])
        },
        'delete': () => {
            emit('delete-mods', selectedMods.value)
        },
        'preview': () => {
            emit('preview-mod', selectedMods.value[0])
        },
        'open-folder': () => {
            emit('open-mod-folder', selectedMods.value[0])
        },
    }

    actions[key]?.()
}

function handleKeyDown(event: KeyboardEvent) {
    if ((event.ctrlKey || event.metaKey) && event.key === 'a') {
        event.preventDefault();
        rows.value.forEach(row => row.toggleSelected(true))
        return;
    }
}

function handleRowDoubleClick(event: MouseEvent, rowIndex: number) {
    // avoid previewing mod when double clicking the checkbox to enable/disable the mod
    if (
        !(event.target instanceof HTMLElement) ||
        ["p-checkbox-input", "conflicts-indicator"].some(cls =>
            (event.target as HTMLElement).classList.contains(cls)
        )
    ) {
        return;
    }


    const row = rows.value[rowIndex]
    emit('preview-mod', row.original)
}

</script>

<template>
    <!-- mods contextmenu -->
    <ContextMenu
        :options="contextMenuItems"
        :show="showDropdown"
        :x="x"
        :y="y"
        @update:show="showDropdown = $event"
        @select="handleSelect"
    />

    <!-- header contextmenu -->

    <div class="flex h-full">
        <div ref="parentRef" class="flex-1 overflow-auto border border-border rounded relative" tabindex="0"
            @keydown="handleKeyDown">
            <div :style="{ height: `${totalSize + 46.39}px` }" class="relative">
                <div class="w-full h-full ">
                    <div class="sticky top-0 z-1 bg-bg-surface grid border-b border-border "
                        :style="{ gridTemplateColumns }">
                        <div v-for="header in table.getFlatHeaders()" :key="header.id" :class="[
                            'p-3 px-2 flex items-center gap-2 transition-colors cursor-pointer font-semibold select-none relative bg-bg-surface hover:bg-interactive-bg-hover!'
                        ]" @click="header.column.getToggleSortingHandler()?.($event)">
                            <FlexRender v-if="!header.isPlaceholder" :render="header.column.columnDef.header"
                                :props="header.getContext()" />

                            <ArrowDownWideNarrow v-if="header.column.getIsSorted() == 'desc'"
                                class="w-[1.25em] h-[1.25em] text-secondary" />
                            <ArrowUpNarrowWide v-if="header.column.getIsSorted() == 'asc'"
                                class="w-[1.25em] h-[1.25em] text-secondary" />

                            <div v-if="header.column.getCanResize()" @dblclick="header.column.resetSize()" @click.stop
                                @mousedown="(event) => {if (!header.column.getIsLastColumn()) header.getResizeHandler()?.(event)}"
                                @touchstart="(event) => {if (!header.column.getIsLastColumn()) header.getResizeHandler()?.(event)}" :class="[
                                    'z-9999 absolute right-0 top-0 h-full w-0.5  opacity-0 hover:opacity-100 select-none touch-none',
                                    !header.column.getIsLastColumn() ? 'hover:bg-accent-primary-hover cursor-col-resize' : '',
                                    header.column.getIsResizing() && !header.column.getIsLastColumn() ? 'bg-accent-primary opacity-100 h-screen!' : ''
                                ]" :style="header.column.getIsResizing() ? {
                                    transform: `translateX(${table.getState().columnSizingInfo.deltaOffset ?? 0}px)`
                                } : undefined">
                            </div>
                        </div>
                    </div>

                    <div v-for="virtualRow in virtualRows" :key="virtualRow.index" :style="{
                        height: `${virtualRow.size}px`,
                        transform: `translateY(${virtualRow.start + 46.39}px)`,
                        gridTemplateColumns
                    }" class="absolute top-0 left-0 min-w-full grid items-center hover:bg-interactive-bg-hover border-b border-border"
                        @contextmenu="handleContextMenu($event, virtualRow.index)"
                        @click="handleRowClick($event, virtualRow.index)"
                        @dblclick="handleRowDoubleClick($event, virtualRow.index)"
                        :class="{ 'bg-accent-primary!': rows[virtualRow.index].getIsSelected() }">
                        <div v-for="cell in rows[virtualRow.index].getVisibleCells()" :key="cell.id"
                            class="p-3 overflow-hidden">
                            <FlexRender :render="cell.column.columnDef.cell" :props="cell.getContext()" />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>