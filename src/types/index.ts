export type SelectOption = { label: string; value: number };
export type MentionOption = SelectOption;
export type BacklogType = SelectOption;
export type ProgressOption = SelectOption;

export type TaskCell = {
  name: string;
  date: number;
  hours: number;
};

export type EditTaskProps = {
  id: number;
  type: 'name' | 'date' | 'hours' | 'pic';
  title: string;
};

export type Participant = {
  id: number;
  alias: string;
};

export type Iteration = {
  id: number;
  title: string;
  goals: string;
  createdBy: string;
  endDate: number;
};

export type Backlog = {
  id: number;
  title: string;
  description: string;
  goals: string;
  priority: number;
  hours: number;
  points: number;
  createdDate: number;
};

export type Task = {
  id: number;
  name: string;
  createdDate: number;
  hours: number;
  actualHours: number;
  progress: string;
  pic: string;
};

export type Logwork = {
  pic: number;
  estimatedHours: number;
  workedHours: number;
  completedDate: number;
};

export * from './snake_case';
