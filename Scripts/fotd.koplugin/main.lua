local ConfirmBox = require("ui/widget/confirmbox")
local DataStorage = require("datastorage")
local Dispatcher = require("dispatcher")
local KeyValuePage = require("ui/widget/keyvaluepage")
local LuaSettings = require("luasettings")
local PowerD = require("device"):getPowerDevice()
local UIManager = require("ui/uimanager")
local WidgetContainer = require("ui/widget/container/widgetcontainer")
local datetime = require("datetime")
local dbg = require("dbg")
local time = require("ui/time")
local _ = require("gettext")
local T = require("ffi/util").template

local FotdWidget = WidgetContainer:extend{
    name = "fotd",
}

function FotdWidget:onDispatcherRegisterActions()
    Dispatcher:registerAction("fotd", {category="none", event="ShowFotd", title=_("Fact of the Day"), device=true, separator=true})
end

function FotdWidget:init()
    if not self.ui or not self.ui.menu then return end
    self:onDispatcherRegisterActions()
    self.ui.menu:registerToMainMenu(self)
end

function FotdWidget:addToMainMenu(menu_items)
    menu_items.fotd = {
        text = _("Fact of the Day"),
        keep_menu_open = true,
        callback = function()
            -- Callback --
        end,
    }
end

function FotdWidget:onShowFotd()
    -- Callback --
end

return FotdWidget
