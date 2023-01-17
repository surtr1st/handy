<script setup lang="ts">
import { h, ref } from 'vue';
import {
  NButton,
  NDataTable,
  DataTableColumns,
  DataTableRowKey,
} from 'naive-ui';
type Song = {
  no: number;
  title: string;
  length: string;
};

type Task = {};
const data: void[] = [];
const createColumns = ({
  play,
}: {
  play: (row: Song) => void;
}): DataTableColumns<Song> => {
  return [
    {
      title: 'No',
      key: 'no',
    },
    {
      title: 'Title',
      key: 'title',
    },
    {
      title: 'Length',
      key: 'length',
    },
    {
      title: 'Action',
      key: 'actions',
      render(row) {
        return h(
          NButton,
          {
            strong: true,
            tertiary: true,
            size: 'small',
            onClick: () => play(row),
          },
          { default: () => 'Play' },
        );
      },
    },
  ];
};

const checkedRowKeys = ref<DataTableRowKey[]>([]);
function handleCheck(rowKeys: DataTableRowKey[]) {
  checkedRowKeys.value = rowKeys;
}
</script>

<template>
  <NDataTable
    :columns="createColumns({ play (row: Song) { } })"
    @update:checked-row-keys="handleCheck"
  />
</template>
