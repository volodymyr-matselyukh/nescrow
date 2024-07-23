'use client';

import { Task } from '@/types/Task';
import { Radio, RadioChangeEvent } from 'antd';
import { FC, useState } from 'react';
import TaskItem from './TaskItem';

enum TaskType {
  FREELANCER_TASK, // a task that I can pick up as a freelancer
  OWNER_TASK, // a task that I create and own and give for a freelancer to work on
}

interface Props {
  myFrellancerTasks: Task[];
  myOwnerTasks: Task[];
}

const options = [
  { label: 'Freelancer', value: TaskType.FREELANCER_TASK },
  { label: 'Owner', value: TaskType.OWNER_TASK },
];

const TasksContainer: FC<Props> = ({ myFrellancerTasks, myOwnerTasks }) => {
  const [taskType, setTaskType] = useState(TaskType.FREELANCER_TASK);

  const onTaskTypeChange = ({ target: { value } }: RadioChangeEvent) => {
    setTaskType(value);
  };

  const getTitle = () => {
    if(taskType === TaskType.FREELANCER_TASK)
    {
      return "My freelancer's tasks:";
    }
    else if(taskType === TaskType.OWNER_TASK){
      return "My owner's tasks:"
    }
  }

  return (
    <div className='flex flex-col gap-4'>
      <Radio.Group
        options={options}
        onChange={onTaskTypeChange}
        value={taskType}
        optionType="button"
        buttonStyle="solid"
        className="text-center"
      />

      <h1 className='font-medium'>{getTitle()}</h1>

      {taskType === TaskType.FREELANCER_TASK &&
        myFrellancerTasks.map((item, index) => (
          <TaskItem key={index} task={item} />
        ))}

      {taskType === TaskType.OWNER_TASK &&
        myOwnerTasks.map((item, index) => (
          <TaskItem key={index} task={item} />
        ))}
    </div>
  );
};

export default TasksContainer;
