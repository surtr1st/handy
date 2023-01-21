import { colors } from './colors';

const primary = {
  colorHoverPrimary: colors.primaryHover,
  textColorPrimary: colors.text,
  textColorHoverPrimary: colors.text,
  colorPrimary: colors.primary,
  borderHoverPrimary: `1px solid ${colors.primaryHover}`,
  colorFocusPrimary: colors.primary,
  borderFocusPrimary: `1px solid ${colors.primaryHover}`,
  textColorFocusPrimary: colors.text,
};

export const buttonTheme = {
  ...primary,
};
