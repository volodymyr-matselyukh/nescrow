import * as yup from 'yup';

import { yupResolver } from '@hookform/resolvers/yup';
import { Button, Form, Input } from 'antd';
import { useForm } from 'react-hook-form';
import { FormItem } from 'react-hook-form-antd';

const MIN_PASSWORD_LENGTH = 8;

const loginSchema = yup
  .object({
    email: yup.string().email().required(),
    password: yup.string().min(MIN_PASSWORD_LENGTH).required(),
    repeat_password: yup
      .string()
      .oneOf([yup.ref('password'), ''], 'Passwords must match'),
  })
  .required();

const SignIn = () => {
  const {
    control,
    handleSubmit,
    
  } = useForm({
    resolver: yupResolver(loginSchema),
  });

  const onFinish = async () => {
    await handleSubmit((values) => {
      console.log('values', values);
    })();
  };

  const onFinishFailed = (errorInfo: any) => {
    console.log('submit failed', errorInfo);
  };

  return (
    <>
      <Form
        className="flex flex-col gap-8"
        name="basic"
        style={{ maxWidth: 600 }}
        initialValues={{ remember: true }}
        onFinish={onFinish}
        onFinishFailed={onFinishFailed}
        autoComplete="off"
      >
        <FormItem
          control={control}
          name="email"
          label="Email *"
          layout="vertical"
        >
          <Input size="large" />
        </FormItem>

        <FormItem
          control={control}
          name="password"
          label="Password *"
          layout="vertical"
        >
          <Input.Password size="large" />
        </FormItem>

        <FormItem
          control={control}
          name="repeat_password"
          label="Repeat password *"
          layout="vertical"
        >
          <Input.Password size="large" />
        </FormItem>

        <Form.Item className="text-right">
          <Button type="primary" htmlType="submit" size="large">
            Sing up
          </Button>
        </Form.Item>
      </Form>
    </>
  );
};

export default SignIn;
