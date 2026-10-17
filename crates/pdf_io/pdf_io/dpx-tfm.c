/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
   the dvipdfmx project team.

   Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/

#include "dpx-tfm.h"

#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "tectonic-tfm.h"

/* Parsing lives in Rust. Diagnostics and aborts stay here so a native
 * longjmp never crosses a Rust frame. */
static void
tfm_flush (const char *error)
{
    const char *text;
    int kind = 0;

    while ((text = tt_tfm_next_diagnostic(&kind)) != NULL) {
        if (kind == 0)
            dpx_message("%s", text);
        else
            dpx_warning("%s", text);
    }
    if (error != NULL)
        _tt_abort("%s", error);
}

void
tfm_reset_global_state (void)
{
    tt_tfm_reset();
}

int
tfm_open (const char *tfm_name, int must_exist)
{
    const char *error = NULL;
    int id = tt_tfm_open(tfm_name, must_exist, dpx_conf.verbose_level, &error);
    tfm_flush(error);
    return id;
}

void
tfm_close_all (void)
{
    tt_tfm_close_all();
}

fixword
tfm_get_fw_width (int font_id, int32_t ch)
{
    const char *error = NULL;
    fixword width = tt_tfm_get_fw_width(font_id, ch, &error);
    tfm_flush(error);
    return width;
}

fixword
tfm_get_fw_height (int font_id, int32_t ch)
{
    const char *error = NULL;
    fixword height = tt_tfm_get_fw_height(font_id, ch, &error);
    tfm_flush(error);
    return height;
}

fixword
tfm_get_fw_depth (int font_id, int32_t ch)
{
    const char *error = NULL;
    fixword depth = tt_tfm_get_fw_depth(font_id, ch, &error);
    tfm_flush(error);
    return depth;
}

double
tfm_get_width (int font_id, int32_t ch)
{
    const char *error = NULL;
    double width = tt_tfm_get_width(font_id, ch, &error);
    tfm_flush(error);
    return width;
}

fixword
tfm_string_width (int font_id, const unsigned char *s, unsigned len)
{
    const char *error = NULL;
    fixword width = tt_tfm_string_width(font_id, s, len, &error);
    tfm_flush(error);
    return width;
}

double
tfm_get_design_size (int font_id)
{
    const char *error = NULL;
    double design = tt_tfm_get_design_size(font_id, &error);
    tfm_flush(error);
    return design;
}

int
tfm_is_jfm (int font_id)
{
    return tt_tfm_is_jfm(font_id);
}

bool
tfm_exists (const char *tfm_name)
{
    return tt_tfm_exists(tfm_name);
}
