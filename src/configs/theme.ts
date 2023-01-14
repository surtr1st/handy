import { GlobalThemeOverrides } from 'naive-ui';
import { common } from './common';
import { buttonTheme } from './button';
import { inputTheme } from './input';
import { colors } from './colors';

export const themeOverrides: GlobalThemeOverrides = {
  common: {
    ...common,
  },
  Button: {
    ...buttonTheme,
  },
  Input: {
    ...inputTheme,
  },
};
