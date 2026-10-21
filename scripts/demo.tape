Output demo.gif
Set Framerate 30
Set PlaybackSpeed 1

Set BorderRadius 10

Set FontSize 14
Set Width 1200
Set Height 800
Set Shell zsh

Hide
Type "../target/release/chess-trainer"
Enter
Sleep 2s

Sleep 0.7s
Enter

Show


# ========================================
# LEARN MODE
# ========================================

# Pawn e2 -> e4 (cursor starts on e4)
Down @0.3s
Down @0.3s
Space @0.4s
Up @0.3s
Up @0.3s
Space @0.4s
Sleep 1s

# Knight g1 -> f3 (cursor starts on e4)
Down @0.3s
Down @0.3s
Down @0.3s
Right @0.3s
Right @0.3s
Space @0.4s

Left @0.3s
Up @0.3s
Up @0.3s
Space @0.4s
Sleep 1s

# Bishop f1 -> c4 (cursor starts on f3)
Down @0.3s
Down @0.3s
Space @0.4s

Left @0.3s
Left @0.3s
Left @0.3s
Up @0.3s
Up @0.3s
Up @0.3s
Space @0.4s
Sleep 1.5s



# ========================================
# SWITCH TO DRILL MODE
# ========================================

# Return to menu and switch mode
Type "b"
Sleep 0.5s
Type "m"
Sleep 0.5s
Space @0.3s
Sleep 1.5s

# ========================================
# DRILL MODE
# ========================================

# Cursor starts on c4 (last position in Learn mode)
# Move pawn e2 -> e4

# c4 -> e2
Right @0.3s
Right @0.3s
Down @0.3s
Down @0.3s
Space @0.4s

# e2 -> e4
Up @0.3s
Up @0.3s
Space @0.4s

# Wait for Black's response
Sleep 1.2s

# ========================================
# INCORRECT MOVE
# ========================================

# Cursor starts on e4
# Select pawn d2

Left @0.3s
Down @0.3s
Down @0.3s
Space @0.4s

# Attempt d2 -> d4
Up @0.3s
Up @0.3s
Space @0.4s

# Display incorrect move feedback
Sleep 2.5s
