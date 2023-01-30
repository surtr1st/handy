<script setup lang="ts">
  import TaskCreator from '../components/TaskCreator.vue';
  import Logwork from '../components/Logwork.vue';
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMounted, onUnmounted, reactive, ref, watch } from 'vue';
  import { SnakeTask, EditTaskProps, TaskCell } from '../types';
  import { useBacklogRoute, targetInvoked, targetLogwork } from '../store';
  import { useFormattedDate, useMessages, useNotifications } from '../helpers';
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
    NSwitch,
  } from 'naive-ui';

  const { notifySuccess, notifyError } = useNotifications();
  const { onError } = useMessages();
  const { backlogId: bid } = useBacklogRoute();

  const taskModal = reactive({
    open: false,
    type: '',
    title: '',
  });

  const taskState = reactive({
    name: '',
    date: 0,
    hours: 0,
  });

  const tasks = ref<Array<SnakeTask>>([]);

  const getTask = (id: number) =>
    tasks.value.find((task: SnakeTask) => task.id === id);

  enum CellChosen {
    NAME = 'name',
    STARTED_DATE = 'date',
    HOURS = 'hours',
  }

  function editTaskIndividually({ id, type, title }: EditTaskProps) {
    if (type === 'name') taskState.name = getTask(id)?.name as string;
    if (type === 'date') taskState.date = getTask(id)?.created_date as number;
    if (type === 'hours') taskState.hours = getTask(id)?.hours as number;
    taskModal.open = !taskModal.open;
    taskModal.type = type;
    taskModal.title = title;

    watch(
      () => taskState.name,
      (newName, oldName) => updateTask(id, newName, CellChosen.NAME),
    );
    watch(
      () => taskState.date,
      (newDate, oldDate) => updateTask(id, newDate, CellChosen.STARTED_DATE),
    );
    watch(
      () => taskState.hours,
      (newHours, oldHours) => updateTask(id, newHours, CellChosen.HOURS),
    );
  }

  function updateTaskName(id: number, value: string) {
    invoke<string>('update_task_name', { id, value })
      .then(() => (targetInvoked.taskAction = !targetInvoked.taskAction))
      .catch((message) => notifyError(message));
  }

  function updateTaskStartedDate(id: number, value: number) {
    invoke<string>('update_task_started_date', { id, value })
      .then(() => (targetInvoked.taskAction = !targetInvoked.taskAction))
      .catch((message) => notifyError(message));
  }

  function updateTaskHours(id: number, value: number) {
    invoke<string>('update_task_hours', { id, value })
      .then(() => (targetInvoked.taskAction = !targetInvoked.taskAction))
      .catch((message) => notifyError(message));
  }

  function updateTask(
    id: number,
    updatedValue: string | number,
    cell: CellChosen,
  ) {
    const { name } = getTask(id) as SnakeTask;

    switch (cell) {
      case CellChosen.STARTED_DATE:
        updateTaskStartedDate(id, updatedValue as number);
        break;

      case CellChosen.HOURS:
        updateTaskHours(id, updatedValue as number);
        break;

      default:
        updateTaskName(
          id,
          !updatedValue || updatedValue.toString().length <= 0
            ? name
            : updatedValue.toString(),
        );
        break;
    }
  }

  function updateTaskAfterLogwork() {
    const id = targetLogwork.value.taskId;
    const workedHours = targetLogwork.value.workedHours;

    invoke<string>('update_task_after_logwork', {
      id,
      workedHours,
    })
      .then(() => (targetInvoked.taskAction = !targetInvoked.taskAction))
      .catch((message) => notifyError(message));
  }

  function removeTask(id: number) {
    invoke<string>('remove_task', { id })
      .then((message) => {
        notifySuccess(message);
        targetInvoked.taskAction = !targetInvoked.backlogAction;
      })
      .catch((message) => notifyError(message));
  }

  function fetchTasks() {
    invoke<Array<SnakeTask>>('get_tasks', { backlogId: bid })
      .then((res) => (tasks.value = res))
      .catch((e) => onError(e));
  }

  function onEnter(event: KeyboardEvent) {
    const ENTER = 'Enter';
    if (event.key === ENTER && taskModal.open) {
      event.preventDefault();
      taskModal.open = false;
    }
  }

  watch(
    () => targetInvoked.taskAction,
    () => fetchTasks(),
  );
  watch(
    () => targetLogwork.value,
    () => updateTaskAfterLogwork(),
  );
  onMounted(() => fetchTasks());
  onUnmounted(() => (tasks.value = []));
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
        <th>Started Date</th>
        <th>Hours</th>
        <th>PIC</th>
        <th
          colspan="2"
          style="text-align: center"
        >
          Progress
        </th>
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
          style="max-width: 100px; word-wrap: break-word"
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
          {{ useFormattedDate(task.started_date) }}
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
          {{ task.worked_hours }}/{{ task.hours }}
        </td>
        <td>
          {{ task.pic }}
        </td>
        <td style="width: 190px">
          <NSpace
            justify="center"
            align="center"
          >
            <Logwork
              :props="{
                pic: task.pic,
                estimatedHours: task.hours,
                startedDate: task.started_date,
                taskId: task.id,
                taskStatus: task.status,
              }"
            />
            <NButton
              primary
              type="error"
              :disabled="task.status"
              @click="removeTask(task.id)"
              >Remove</NButton
            >
          </NSpace>
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
        maxlength="50"
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
        min="0"
        max="8"
        @keydown="onEnter"
      />
    </NCard>
  </NModal>
</template>
