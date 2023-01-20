<script setup lang="ts">
import TaskCreator from '../components/TaskCreator.vue';
import Logwork from '../components/Logwork.vue';
import { useFormattedDate } from '../constants';
import { Task, EditTaskProps } from '../types';
import { reactive, ref } from 'vue';
import {
  NButton,
  NGrid,
  NGi,
  NCard,
  NSpace,
  NText,
  NTable,
  NModal,
  NInput,
  NDatePicker,
  NInputNumber,
  NSelect,
  NSwitch,
} from 'naive-ui';

const taskModal = reactive({
  open: false,
  type: '',
  title: '',
});

const taskState = reactive({
  name: '',
  date: 0,
  hours: 0,
  pic: '',
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

const tasks = ref<Array<Task>>([
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

// Function
const getTask = (id: number) =>
  tasks.value.find((task: Task) => task.id === id);

function editTaskIndividually({ id, type, title }: EditTaskProps) {
  if (type === 'name') taskState.name = getTask(id)?.name as string;
  if (type === 'date') taskState.date = getTask(id)?.createdDate as number;
  if (type === 'hours') taskState.hours = getTask(id)?.hours as number;
  if (type === 'pic') taskState.pic = getTask(id)?.pic as string;
  taskModal.open = !taskModal.open;
  taskModal.type = type;
  taskModal.title = title;
}

function onEnter(event: KeyboardEvent) {
  const ENTER = 'Enter';
  if (event.key === ENTER && taskModal.open) {
    event.preventDefault();
    taskModal.open = false;
  }
}
</script>

<template>
  <NGrid
    responsive="screen"
    :cols="12"
  >
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
        <th></th>
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
        <td
          @dblclick="
            editTaskIndividually({
              id: task.id,
              type: 'name',
              title: 'Edit Name',
            })
          "
        >
          <NText strong>{{ task.name }}</NText>
        </td>
        <td
          @dblclick="
            editTaskIndividually({
              id: task.id,
              type: 'date',
              title: 'Edit Date',
            })
          "
        >
          {{ useFormattedDate(task.createdDate) }}
        </td>
        <td
          @dblclick="
            editTaskIndividually({
              id: task.id,
              type: 'hours',
              title: 'Edit Hours',
            })
          "
        >
          {{ task.actualHours }}/{{ task.hours }}
        </td>
        <td
          @dblclick="
            editTaskIndividually({
              id: task.id,
              type: 'pic',
              title: 'Edit Person-In-Charge',
            })
          "
        >
          {{ task.pic }}
        </td>
        <td style="width: 100px">
          <Logwork :props="{ pic: task.pic, estimatedHours: task.hours }" />
        </td>
        <td>
          <NButton
            primary
            type="primary"
            >Edit</NButton
          >
        </td>
      </tr>
    </tbody>
  </NTable>
  <NModal v-model:show="taskModal.open">
    <NCard
      style="width: 600px"
      :title="taskModal.title"
      role="dialog"
    >
      <NInput
        v-show="taskModal.type === 'name'"
        v-model:value="taskState.name"
        @keydown="onEnter"
      />
      <NDatePicker
        v-show="taskModal.type === 'date'"
        v-model:value="taskState.date"
        @keydown="onEnter"
      />
      <NInputNumber
        v-show="taskModal.type === 'hours'"
        v-model:value="taskState.hours"
        @keydown="onEnter"
      />
      <NSelect
        v-show="taskModal.type === 'pic'"
        :options="[{ label: taskState.pic, value: taskState.pic }]"
      />
    </NCard>
  </NModal>
</template>
