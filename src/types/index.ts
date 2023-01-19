export type EditTaskProps = {
  id: number;
  type: 'name' | 'date' | 'hours' | 'pic';
  title: string;
};

export type Iteration = {
  id: number;
  title: string;
  goals: string;
  participants: number;
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
