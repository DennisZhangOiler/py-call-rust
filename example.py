import cv2, py_call_rust

frame_as_ndarr = py_call_rust.capture_in_rust()
cv2.imwrite("./before.png", frame_as_ndarr)
resized = cv2.resize(frame_as_ndarr, (480, 320))
cv2.imwrite("./after.png", resized)