export type Task = {
  id: number;
  name: string;
  createdDate: number;
  hours: number;
  actualHours: number;
  progress: string;
  pic: string;
};

export type EditTaskProps = {
  id: number;
  type: 'name' | 'date' | 'hours' | 'pic';
  title: string;
};
