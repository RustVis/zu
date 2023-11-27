// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

#![allow(clippy::too_many_lines)]

use yew::{function_component, html, Html};
use zu::boxed::Box;
use zu::styles::input_type::InputType;
use zu::styles::label_variant::LabelVariant;
use zu::text_field::TextField;

use crate::components::demo_box::DemoBox;

fn basic_section() -> Html {
    html! {
        <>
        <h2>{"Basic TextField"}</h2>
        <p>{"The TextField wrapper component is a complete form control including a label, input, and help text. \
        It comes with three variants: outlined (default), filled, and standard."}</p>
        <DemoBox>
        <TextField id="outlined-basic" label="Outlined" variant={LabelVariant::Outlined} />
        <TextField id="filled-basic" label="Filled" variant={LabelVariant::Filled} />
        <TextField id="standard-basic" label="Standard" variant={LabelVariant::Standard} />
        </DemoBox>
        </>
    }
}

fn form_props_section() -> Html {
    html! {
        <>
        <h2>{"Form props"}</h2>
        <p>{"Standard form attributes are supported e.g. required, disabled, type, etc. as well as a helperText \
        which is used to give context about a field's input, such as how the input will be used."}</p>
        <DemoBox>
        <Box
            component="form"
            style="& .ZuTextField-root: margin: 1px; width: 25ch;"
            no_validate={true}
            auto_complete={false}
        >
        <div>
            <TextField
                required={true}
                id="outlined-required"
                label="Required"
                default_value="Hello World"
            />
            <TextField
                disabled={true}
                id="outlined-disabled"
                label="Disabled"
                default_value="Hello World"
            />
            <TextField
                id="outlined-password-input"
                label="Password"
                input_type={InputType::Password}
                auto_complete="current-password"
            />
            <TextField
                id="outlined-read-only-input"
                label="Read Only"
                default_value="Hello World"
                input_read_only={true}
            />
            <TextField
                id="outlined-number"
                label="Number"
                input_type={InputType::Number}
                input_shrink={true}
            />
            <TextField
                id="outlined-search"
                label="Search field"
                input_type={InputType::Search}
            />
            <TextField
                id="outlined-helperText"
                label="Helper text"
                default_value="Default Value"
                helper_text="Some important text"
            />
        </div>
        <div>
            <TextField
                required={true}
                id="filled-required"
                label="Required"
                default_value="Hello World"
                variant={LabelVariant::Filled}
            />
            <TextField
                disabled={true}
                id="filled-disabled"
                label="Disabled"
                default_value="Hello World"
                variant={LabelVariant::Filled}
            />
            <TextField
                id="filled-password-input"
                label="Password"
                input_type={InputType::Password}
                auto_complete="current-password"
                variant={LabelVariant::Filled}
            />
            <TextField
                id="filled-read-only-input"
                label="Read Only"
                default_value="Hello World"
                input_read_only={true}
                variant={LabelVariant::Filled}
            />
            <TextField
                id="filled-number"
                label="Number"
                input_type={InputType::Number}
                input_shrink={true}
                variant={LabelVariant::Filled}
            />
            <TextField
                id="filled-search"
                label="Search field"
                input_type={InputType::Search}
                variant={LabelVariant::Filled}
            />
            <TextField
                id="filled-helperText"
                label="Helper text"
                default_value="Default Value"
                helper_text="Some important text"
                variant={LabelVariant::Filled}
            />
        </div>
        <div>
            <TextField
                required={true}
                id="standard-required"
                label="Required"
                default_value="Hello World"
                variant={LabelVariant::Standard}
            />
            <TextField
                disabled={true}
                id="standard-disabled"
                label="Disabled"
                default_value="Hello World"
                variant={LabelVariant::Standard}
            />
            <TextField
                id="standard-password-input"
                label="Password"
                input_type={InputType::Password}
                auto_complete="current-password"
                variant={LabelVariant::Standard}
            />
            <TextField
                id="standard-read-only-input"
                label="Read Only"
                default_value="Hello World"
                input_read_only={true}
                variant={LabelVariant::Standard}
            />
            <TextField
                id="standard-number"
                label="Number"
                input_type={InputType::Number}
                input_shrink={true}
                variant={LabelVariant::Standard}
            />
            <TextField
                id="standard-search"
                label="Search field"
                input_type={InputType::Search}
                variant={LabelVariant::Standard}
            />
            <TextField
                id="standard-helperText"
                label="Helper text"
                default_value="Default Value"
                helper_text="Some important text"
                variant={LabelVariant::Standard}
            />
        </div>
      </Box>
      </DemoBox>
      </>
    }
}

/*

fn select_section() -> Html {
  html!{
    <>
    <h2>{"Select"}</h2>
    <p>{"The select prop makes the text field use the Select component internally."}</p>
    <DemoBox>
      <Box
      component="form"
      sx={{
        '& .MuiTextField-root': { m: 1, width: '25ch' },
      }}
      noValidate
      autoComplete="off"
    >
      <div>
        <TextField
          id="outlined-select-currency"
          select
          label="Select"
          defaultValue="EUR"
          helperText="Please select your currency"
        >
          {currencies.map((option) => (
            <MenuItem key={option.value} value={option.value}>
              {option.label}
            </MenuItem>
          ))}
        </TextField>
        <TextField
          id="outlined-select-currency-native"
          select
          label="Native select"
          defaultValue="EUR"
          SelectProps={{
            native: true,
          }}
          helperText="Please select your currency"
        >
          {currencies.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </TextField>
      </div>
      <div>
        <TextField
          id="filled-select-currency"
          select
          label="Select"
          defaultValue="EUR"
          helperText="Please select your currency"
          variant="filled"
        >
          {currencies.map((option) => (
            <MenuItem key={option.value} value={option.value}>
              {option.label}
            </MenuItem>
          ))}
        </TextField>
        <TextField
          id="filled-select-currency-native"
          select
          label="Native select"
          defaultValue="EUR"
          SelectProps={{
            native: true,
          }}
          helperText="Please select your currency"
          variant="filled"
        >
          {currencies.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </TextField>
      </div>
      <div>
        <TextField
          id="standard-select-currency"
          select
          label="Select"
          defaultValue="EUR"
          helperText="Please select your currency"
          variant="standard"
        >
          {currencies.map((option) => (
            <MenuItem key={option.value} value={option.value}>
              {option.label}
            </MenuItem>
          ))}
        </TextField>
        <TextField
          id="standard-select-currency-native"
          select
          label="Native select"
          defaultValue="EUR"
          SelectProps={{
            native: true,
          }}
          helperText="Please select your currency"
          variant="standard"
        >
          {currencies.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </TextField>
      </div>
    </Box>
    </DemoBox>
    </>
  }
}

fn icons_section() -> Html {
  html!{
    <>
    <h2>{"Icons"}</h2>
    <p>{"There are multiple ways to display an icon with a text field."}</p>
    <DemoBox>
    <>
      <Box sx={{ '& > :not(style)': { m: 1 } }}>
      <FormControl variant="standard">
        <InputLabel htmlFor="input-with-icon-adornment">
          With a start adornment
        </InputLabel>
        <Input
          id="input-with-icon-adornment"
          startAdornment={
            <InputAdornment position="start">
              <AccountCircle />
            </InputAdornment>
          }
        />
      </FormControl>
      <TextField
        id="input-with-icon-textfield"
        label="TextField"
        InputProps={{
          startAdornment: (
            <InputAdornment position="start">
              <AccountCircle />
            </InputAdornment>
          ),
        }}
        variant="standard"
      />
      <Box sx={{ display: 'flex', alignItems: 'flex-end' }}>
        <AccountCircle sx={{ color: 'action.active', mr: 1, my: 0.5 }} />
        <TextField id="input-with-sx" label="With sx" variant="standard" />
      </Box>
    </Box>
    </>
    </DemoBox>
    </>
  }
}

fn input_adornments_subsection() -> Html {
  html!{
    <>
    <h3>{"Input Adornments"}</h3>
    <p>{"The main way is with an InputAdornment. This can be used to add a prefix, a suffix, or an action \
    to an input. For instance, you can use an icon button to hide or reveal the password."}</p>
    <DemoBox>
    <Box sx={{ display: 'flex', flexWrap: 'wrap' }}>
      <div>
        <TextField
          label="With normal TextField"
          id="outlined-start-adornment"
          sx={{ m: 1, width: '25ch' }}
          InputProps={{
            startAdornment: <InputAdornment position="start">kg</InputAdornment>,
          }}
        />
        <FormControl sx={{ m: 1, width: '25ch' }} variant="outlined">
          <OutlinedInput
            id="outlined-adornment-weight"
            endAdornment={<InputAdornment position="end">kg</InputAdornment>}
            aria-describedby="outlined-weight-helper-text"
            inputProps={{
              'aria-label': 'weight',
            }}
          />
          <FormHelperText id="outlined-weight-helper-text">Weight</FormHelperText>
        </FormControl>
        <FormControl sx={{ m: 1, width: '25ch' }} variant="outlined">
          <InputLabel htmlFor="outlined-adornment-password">Password</InputLabel>
          <OutlinedInput
            id="outlined-adornment-password"
            type={showPassword ? 'text' : 'password'}
            endAdornment={
              <InputAdornment position="end">
                <IconButton
                  aria-label="toggle password visibility"
                  onClick={handleClickShowPassword}
                  onMouseDown={handleMouseDownPassword}
                  edge="end"
                >
                  {showPassword ? <VisibilityOff /> : <Visibility />}
                </IconButton>
              </InputAdornment>
            }
            label="Password"
          />
        </FormControl>
        <FormControl fullWidth sx={{ m: 1 }}>
          <InputLabel htmlFor="outlined-adornment-amount">Amount</InputLabel>
          <OutlinedInput
            id="outlined-adornment-amount"
            startAdornment={<InputAdornment position="start">$</InputAdornment>}
            label="Amount"
          />
        </FormControl>
      </div>
      <div>
        <TextField
          label="With normal TextField"
          id="filled-start-adornment"
          sx={{ m: 1, width: '25ch' }}
          InputProps={{
            startAdornment: <InputAdornment position="start">kg</InputAdornment>,
          }}
          variant="filled"
        />
        <FormControl sx={{ m: 1, width: '25ch' }} variant="filled">
          <FilledInput
            id="filled-adornment-weight"
            endAdornment={<InputAdornment position="end">kg</InputAdornment>}
            aria-describedby="filled-weight-helper-text"
            inputProps={{
              'aria-label': 'weight',
            }}
          />
          <FormHelperText id="filled-weight-helper-text">Weight</FormHelperText>
        </FormControl>
        <FormControl sx={{ m: 1, width: '25ch' }} variant="filled">
          <InputLabel htmlFor="filled-adornment-password">Password</InputLabel>
          <FilledInput
            id="filled-adornment-password"
            type={showPassword ? 'text' : 'password'}
            endAdornment={
              <InputAdornment position="end">
                <IconButton
                  aria-label="toggle password visibility"
                  onClick={handleClickShowPassword}
                  onMouseDown={handleMouseDownPassword}
                  edge="end"
                >
                  {showPassword ? <VisibilityOff /> : <Visibility />}
                </IconButton>
              </InputAdornment>
            }
          />
        </FormControl>
        <FormControl fullWidth sx={{ m: 1 }} variant="filled">
          <InputLabel htmlFor="filled-adornment-amount">Amount</InputLabel>
          <FilledInput
            id="filled-adornment-amount"
            startAdornment={<InputAdornment position="start">$</InputAdornment>}
          />
        </FormControl>
      </div>
      <div>
        <TextField
          label="With normal TextField"
          id="standard-start-adornment"
          sx={{ m: 1, width: '25ch' }}
          InputProps={{
            startAdornment: <InputAdornment position="start">kg</InputAdornment>,
          }}
          variant="standard"
        />
        <FormControl variant="standard" sx={{ m: 1, mt: 3, width: '25ch' }}>
          <Input
            id="standard-adornment-weight"
            endAdornment={<InputAdornment position="end">kg</InputAdornment>}
            aria-describedby="standard-weight-helper-text"
            inputProps={{
              'aria-label': 'weight',
            }}
          />
          <FormHelperText id="standard-weight-helper-text">Weight</FormHelperText>
        </FormControl>
        <FormControl sx={{ m: 1, width: '25ch' }} variant="standard">
          <InputLabel htmlFor="standard-adornment-password">Password</InputLabel>
          <Input
            id="standard-adornment-password"
            type={showPassword ? 'text' : 'password'}
            endAdornment={
              <InputAdornment position="end">
                <IconButton
                  aria-label="toggle password visibility"
                  onClick={handleClickShowPassword}
                  onMouseDown={handleMouseDownPassword}
                >
                  {showPassword ? <VisibilityOff /> : <Visibility />}
                </IconButton>
              </InputAdornment>
            }
          />
        </FormControl>
        <FormControl fullWidth sx={{ m: 1 }} variant="standard">
          <InputLabel htmlFor="standard-adornment-amount">Amount</InputLabel>
          <Input
            id="standard-adornment-amount"
            startAdornment={<InputAdornment position="start">$</InputAdornment>}
          />
        </FormControl>
      </div>
    </Box>
    </DemoBox>
    </>
  }
}

fn sizes_section() -> Html {
  html!{
    <>
    <h2>{"Sizes"}</h2>
    <p>{"Fancy smaller inputs? Use the size prop."}</p>
    <DemoBox>
     <Box
      component="form"
      sx={{
        '& .MuiTextField-root': { m: 1, width: '25ch' },
      }}
      noValidate
      autoComplete="off"
    >
      <div>
        <TextField
          label="Size"
          id="outlined-size-small"
          defaultValue="Small"
          size="small"
        />
        <TextField label="Size" id="outlined-size-normal" defaultValue="Normal" />
      </div>
      <div>
        <TextField
          label="Size"
          id="filled-size-small"
          defaultValue="Small"
          variant="filled"
          size="small"
        />
        <TextField
          label="Size"
          id="filled-size-normal"
          defaultValue="Normal"
          variant="filled"
        />
      </div>
      <div>
        <TextField
          label="Size"
          id="standard-size-small"
          defaultValue="Small"
          size="small"
          variant="standard"
        />
        <TextField
          label="Size"
          id="standard-size-normal"
          defaultValue="Normal"
          variant="standard"
        />
      </div>
    </Box>
    </DemoBox>

    <p>{"The filled variant input height can be further reduced by rendering the label outside of it."}</p>
    <DemoBox>
     <Stack
      component="form"
      sx={{
        width: '25ch',
      }}
      spacing={2}
      noValidate
      autoComplete="off"
    >
      <TextField
        hiddenLabel
        id="filled-hidden-label-small"
        defaultValue="Small"
        variant="filled"
        size="small"
      />
      <TextField
        hiddenLabel
        id="filled-hidden-label-normal"
        defaultValue="Normal"
        variant="filled"
      />
    </Stack>
    </DemoBox>
    </>
  }
}

fn margin_section() -> Html {
  html!{
    <>
    <h2>{"Margin"}</h2>
    <p>{"The margin prop can be used to alter the vertical spacing of the text field. Using none (default) \
    doesn't apply margins to the FormControl whereas dense and normal do."}</p>
    <DemoBox>
     <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        '& .MuiTextField-root': { width: '25ch' },
      }}
    >
      <RedBar />
      <TextField label={'margin="none"'} id="margin-none" />
      <RedBar />
      <TextField label={'margin="dense"'} id="margin-dense" margin="dense" />
      <RedBar />
      <TextField label={'margin="normal"'} id="margin-normal" margin="normal" />
      <RedBar />
    </Box>
    </DemoBox>
    </>
  }
}

      */

#[function_component(TextFieldPage)]
pub fn text_field_page() -> Html {
    html! {
        <>
        <h1>{"Text Field"}</h1>
        <p>{"Text fields allow users to enter text into a UI. They typically appear in forms and dialogs."}</p>

        {basic_section()}
        {form_props_section()}
        // {select_section()}
        // {icons_section()}
        // {input_adornments_subsection()}

        </>
    }
}
