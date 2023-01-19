<script setup lang="ts">
import dayjs from 'dayjs';
import { Task } from '../types';
import { CSSProperties, reactive, ref } from 'vue';
import { NGrid, NGi, NCard, NSpace, NText, NTable, NSwitch } from 'naive-ui';
import TaskCreator from '../components/TaskCreator.vue';
import TaskNameModal from '../components/TaskNameModal.vue';

const taskState = reactive({
  name: '',
  date: 0,
  hours: 0,
  pic: '',
});
const taskModal = reactive({
  open: false,
  cellNameOpen: false,
  cellDateOpen: false,
  cellHoursOpen: false,
  cellPicOpen: false,
});
const task = reactive<Task>({
  id: 0,
  name: '',
  createdDate: 0,
  hours: 0,
  actualHours: 0,
  progress: '',
  pic: '',
});
const tasks = ref<Task[]>([
  {
    id: 1,
    name: 'A du dark wa 123 456 Wy Seg',
    createdDate: 1461110400000,
    hours: 1,
    actualHours: 0,
    pic: '@chi.tr',
    progress: 'Undone',
  },
  {
    id: 2,
    name: 'A du dark wa 13 46 Wy Seg',
    createdDate: 1461110400000,
    hours: 1,
    actualHours: 0,
    pic: '@chi.tr',
    progress: 'Undone',
  },
]);

const railStyle = ({
  focused,
  checked,
}: {
  focused: boolean;
  checked: boolean;
}) => {
  const style: CSSProperties = {};
  style.fontWeight = 'bolder';
  if (checked) {
    style.background = 'rgb(16, 185, 129)';
    if (focused) {
      style.boxShadow = '0 0 0 2px #d0305040';
    }
  } else {
    style.background = 'rgb(225, 29, 72)';
    if (focused) {
      style.boxShadow = '0 0 0 2px #2080f040';
    }
  }
  return style;
};

// Function
const getTask = (id: number) =>
  tasks.value.find((task: Task) => task.id === id);

function editTask(id: number) {
  const task = getTask(id);
}

function editTaskName(id: number) {
  taskState.name = getTask(id)?.name as string;
  taskModal.cellNameOpen = !taskModal.cellNameOpen;
}

function editTaskDate(id: number) {
  taskState.date = getTask(id)?.createdDate as number;
  taskModal.cellDateOpen = !taskModal.cellDateOpen;
}

function editTaskHours(id: number) {
  taskState.hours = getTask(id)?.createdDate as number;
  taskModal.cellHoursOpen = !taskModal.cellHoursOpen;
}

function editTaskPIC(id: number) {
  taskState.pic = getTask(id)?.pic as string;
  taskModal.cellPicOpen = !taskModal.cellPicOpen;
}

function onEnter(event: KeyboardEvent) {
  const ENTER = 'Enter';
  if (event.key === ENTER && taskModal.cellNameOpen) {
    event.preventDefault();
    taskModal.cellNameOpen = false;
  }
}
</script>

<template>
  <NGrid :cols="12">
    <NGi :span="11">
      <NCard
        title="Total Hour"
        style="margin-bottom: 0.7rem"
      >
        <template #header-extra>
          <NSpace>
            <NText
              strong
              type="success"
            >
              0 / 100
            </NText>
            <NText strong> Auto-calculating Time? </NText>
            <NSwitch>
              <template #checked>Yes</template>
              <template #unchecked>No</template>
            </NSwitch>
          </NSpace>
        </template>
      </NCard>
    </NGi>
    <NGi :span="1">
      <TaskCreator />
    </NGi>
  </NGrid>

  <NTable>
    <thead>
      <tr>
        <th>Id</th>
        <th>Name</th>
        <th>Created Date</th>
        <th>Hours</th>
        <th>PIC</th>
        <th>Progress</th>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="task in tasks"
        :key="task.id"
      >
        <td>
          <NText strong>{{ task.id }}</NText>
        </td>
        <td @dblclick="editTaskName(task.id)">
          <NText strong>{{ task.name }}</NText>
          <TaskNameModal
            :value="taskState.name"
            :close-on-enter="onEnter"
          />
        </td>
        <td @dblclick="editTaskDate(task.id)">
          {{ dayjs(new Date(task.createdDate)).format('DD-MM-YYYY') }}
        </td>
        <td @dblclick="editTaskHours(task.id)">
          {{ task.actualHours }}/{{ task.hours }}
        </td>
        <td @dblclick="editTaskPIC(task.id)">{{ task.pic }}</td>
        <td>
          <NSwitch
            size="large"
            :rail-style="railStyle"
          >
            <template #checked> Done </template>
            <template #unchecked> Undone </template>
          </NSwitch>
        </td>
      </tr>
    </tbody>
  </NTable>
</template>
