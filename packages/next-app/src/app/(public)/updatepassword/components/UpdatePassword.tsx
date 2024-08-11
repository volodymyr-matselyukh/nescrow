'use client';

import * as yup from 'yup';

import { updatePassword } from '@/actions/updatePassword';
import usePageNavigationStore from '@/store/pageNavigationStore';
import { yupResolver } from '@hookform/resolvers/yup';
import { Button, Form, Input, notification } from 'antd';
import { useRouter } from 'next/navigation';
import { FC, useState } from 'react';
import { useForm } from 'react-hook-form';
import { FormItem } from 'react-hook-form-antd';

const MIN_PASSWORD_LENGTH = 8;

const loginSchema = yup
  .object({
    password: yup.string().min(MIN_PASSWORD_LENGTH).required(),
    repeat_password: yup
      .string()
      .oneOf([yup.ref('password')], 'Passwords must match')
      .required(),
  })
  .required();

interface Props {
  code: string;
}

const UpdatePassword: FC<Props> = ({ code }) => {
  const [api, contextHolder] = notification.useNotification();

  const [isLoading, setIsLoading] = useState(false);
  const { setIsNavigating } = usePageNavigationStore();

  const router = useRouter();

  const { control, handleSubmit } = useForm({
    resolver: yupResolver(loginSchema),
  });

  const onFinish = async () => {
    await handleSubmit(async (values) => {
      setIsLoading(true);
      try {
        const result = await updatePassword(code, values.password);
        console.log(result);

        setIsNavigating(true);
        if (result.isSuccess) {
          api.success({ message: 'Password updated' });
          router.push('/login');
        } else {
          router.push(`/updatepassword?message=${result.message}&code=${code}`);
        }
      } finally {
        setIsLoading(false);
        setIsNavigating(false);
      }
    })();
  };

  const onFinishFailed = (errorInfo: any) => {
    console.log('submit failed', errorInfo);
  };

  return (
    <>
      <Form
        className="flex flex-col"
        name="basic"
        style={{ maxWidth: 600 }}
        initialValues={{ remember: true }}
        onFinish={onFinish}
        onFinishFailed={onFinishFailed}
        autoComplete="off"
        layout="vertical"
      >
        {contextHolder}
        <FormItem
          control={control}
          name="password"
          label="Password *"
          layout="vertical"
        >
          <Input.Password size="large" />
        </FormItem>

        <FormItem
          className="block"
          control={control}
          name="repeat_password"
          label="Repeat password *"
          layout="vertical"
        >
          <Input.Password size="large" />
        </FormItem>

        <Form.Item className="mb-0 block text-right" layout="vertical">
          <Button
            type="primary"
            htmlType="submit"
            size="large"
            loading={isLoading}
            disabled={isLoading}
          >
            Update password
          </Button>
        </Form.Item>
      </Form>
    </>
  );
};

export default UpdatePassword;
