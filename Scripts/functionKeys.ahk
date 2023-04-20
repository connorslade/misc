; Garbage AHK Code
; By Connor Slade

; Use Alt + Shift + (1, 2, 3, 4, 5, 6, 7, 8, 9, 0, -, =) as function keys
; (My keyborad dosent have function keys)
!+1::
    Send, {F1}
return

!+2::
    Send, {F2}
return

!+3::
    Send, {F3}
return

!+4::
    Send, {F4}
return

!+5::
    Send, {F5}
return

!+6::
    Send, {F6}
return

!+7::
    Send, {F7}
return

!+8::
    Send, {F8}
return

!+9::
    Send, {F9}
return

!+0::
    Send, {F10}
return

!+-::
    Send, {F11}
return

!+=::
    Send, {F12}
return

; Media Keys
; Guess what! My keyboard also dosent have Media Keys
!+NumpadEnter::
    Send, {Media_Play_Pause}
return

!+NumpadAdd::
    Send, {Media_Next}
return

!+NumpadSub::
    Send, {Media_Prev}
return

!+NumpadMult::
    Send, {Volume_Mute}
return

; Misc
; Just some random things

; Lock PC with Alt + Shift + Return
!+Enter::
    DllCall("LockWorkStation")
return

; Screenshot with Alt + Shift + Numpad /
!+NumpadDiv::
    Send, >!>{PrintScreen}
return

!+NumpadDel::
!+NumpadDot::
    Send, {NumLock}
return
