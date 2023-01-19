import { Task } from '../types';

class TaskBuilder {
  private id?: number;
  private name: string;
  private createdDate: number;
  private hours: number;
  private actualHours: number;
  private progress: string;
  private pic: string;

  constructor(
    id: number,
    name: string,
    createdDate: number,
    hours: number,
    actualHours: number,
    progress: string,
    pic: string,
  ) {
    this.id = id;
    this.name = name;
    this.createdDate = createdDate;
    this.hours = hours;
    this.actualHours = actualHours;
    this.progress = progress;
    this.pic = pic;
  }
}

export function useTask(tasks: Array<Task>) {
  const getTask = (id: number) => tasks.find((task) => task.id === id);

  const editName = (value: string) => {};

  const editDate = (value: number) => value;

  const editHours = (value: number) => value;

  const editPic = (value: string) => value;

  const createTask = ({}: Task) => {};

  const editTask = ({
    name,
    date,
    hours,
    pic,
  }: {
    name: string;
    date: number;
    hours: number;
    pic: string;
  }) => {};

  return {
    getTask,
    editName,
    editDate,
    editHours,
    editPic,
  };
}
