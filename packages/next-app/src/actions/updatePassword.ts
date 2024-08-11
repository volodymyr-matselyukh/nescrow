'use server';

import { getErrorResponse } from '@/utils/supabase/processSupabaseErrors';
import { createClient } from '@/utils/supabase/serverClient';

export const updatePassword = async (code: string, newPassword: string) => {
  const supabase = createClient();

  try {
    const session = await supabase.auth.getSession();

    if (!session) {
      const { error: exchangeError } =
        await supabase.auth.exchangeCodeForSession(code);

      if (exchangeError) {
        return {
          isSuccess: false,
          message: 'Password update code is wrong or expired',
        };
      }
    }

    const { error } = await supabase.auth.updateUser({
      password: newPassword,
    });

    if (error?.code === 'same_password') {
      return {
        isSuccess: false,
        message: 'New password should differ from the old one',
      };
    }

    return getErrorResponse(error);
  } catch (error: any) {
    console.log('Update password error', error);

    if (error?.code === 'validation_failed') {
      return {
        isSuccess: false,
        message: 'Password update code is wrong or expired',
      };
    }

    return {
      isSuccess: false,
      message: 'Update password error. Try later.',
    };
  }
};
