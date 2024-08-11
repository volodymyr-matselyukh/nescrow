'use client';

import * as yup from 'yup';

import { resetPassword } from '@/actions/resetPassword';
import { yupResolver } from '@hookform/resolvers/yup';
import { Button, Form, Input, notification, Typography } from 'antd';
import { useRouter } from 'next/navigation';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { FormItem } from 'react-hook-form-antd';

const { Text } = Typography;

const cleanUrl = window.location.origin + window.location.pathname;

const loginSchema = yup
  .object({
    email: yup.string().email().required(),
  })
  .required();

const ResetPassword = () => {
  const [api, contextHolder] = notification.useNotification();

  const router = useRouter();

  const [isLoading, setIsLoading] = useState(false);
  const { control, handleSubmit } = useForm({
    resolver: yupResolver(loginSchema),
  });

  const onFinish = async () => {
    await handleSubmit(async (values) => {
      router.replace(cleanUrl);
      setIsLoading(true);
      try {
        const result = await resetPassword(values.email);
        if (result.isSuccess) {
          api.success({ message: 'Reset password link set to the email' });
        } else {
          router.push(`/resetpassword?message=${result.message}`);
        }
      } catch (error) {
        router.push('/resetpassword?message=Problem with password reset');
      } finally {
        setIsLoading(false);
      }
    })();
  };

  return (
    <Form onFinish={onFinish} className="flex flex-col">
      {contextHolder}
      <div className="h-max">
        <label>
          <FormItem
            className="[&_.ant-col]:min-h-max"
            control={control}
            name="email"
            label="Email *"
            layout="vertical"
          >
            <Input size="large" />
          </FormItem>
        </label>
      </div>

      <div>
        <Text type="secondary">
          A reset password link will be sent to the provided email if such is
          registered in the system.
        </Text>
      </div>

      <Button
        type="primary"
        htmlType="submit"
        size="large"
        className="mt-3 self-end"
        loading={isLoading}
        disabled={isLoading}
      >
        Reset password
      </Button>
    </Form>
  );
};

export default ResetPassword;
