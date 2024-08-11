'use server';

import { getErrorResponse } from '@/utils/supabase/processSupabaseErrors';
import { createClient } from '@/utils/supabase/serverClient';

const BASE_URL = process.env.base_url;

export const resetPassword = async (email: string) => {
  const supabase = createClient();

  const { data, error } = await supabase.auth.resetPasswordForEmail(email, {
    redirectTo: `${BASE_URL}/updatepassword`,
  });

  console.log("data", data);

  return getErrorResponse(error);
};
