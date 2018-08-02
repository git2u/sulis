frag_radius = 4.0

function on_activate(parent, ability)
  targets = parent:targets()
  
  targeter = parent:create_targeter(ability)
  targeter:set_free_select(12.0)
  targeter:set_free_select_must_be_passable("1by1")
  targeter:set_shape_object_size("7by7round")
  targeter:add_all_effectable(targets)
  targeter:activate()
end

function on_target_select(parent, ability, targets)
  selected_point = targets:selected_point()
  speed = 20.0
  dist = parent:dist_to_point(selected_point)
  duration = dist / speed
  vx = (selected_point.x - parent:center_x()) / duration
  vy = (selected_point.y - parent:center_y()) / duration
  
  cb = ability:create_callback(parent)
  cb:add_targets(targets)
  cb:set_on_anim_complete_fn("create_explosion")
  
  gen = parent:create_anim("particles/circle12", duration)
  gen:set_color(gen:param(0.5), gen:param(0.5), gen:param(0.5))
  gen:set_position(gen:param(parent:center_x(), vx), gen:param(parent:center_y(), vy))
  gen:set_particle_size_dist(gen:fixed_dist(0.7), gen:fixed_dist(0.7))
  gen:set_particle_position_dist(gen:dist_param(gen:fixed_dist(0.0), gen:fixed_dist(-vx / 5.0)),
    gen:dist_param(gen:fixed_dist(0.0), gen:fixed_dist(-vy / 5.0)))
  gen:set_completion_callback(cb)
  gen:activate()
  
  ability:activate(parent)
end

function create_explosion(parent, ability, targets)
  duration = 1.2
  
  position = targets:selected_point()
  
  gen = target:create_anim("burst", 0.15)
  gen:set_position(gen:param(position.x - 4.0), gen:param(position.y - 4.0))
  gen:set_particle_size_dist(gen:fixed_dist(8.0), gen:fixed_dist(8.0))
  
  targets = targets:to_table()
  for i = 1, #targets do
    cb = ability:create_callback(parent)
	cb:add_target(targets[i])
	cb:set_on_anim_update_fn("attack_target")
    gen:add_callback(cb, 0.1)
  end
  
  gen:activate()
end

function attack_target(parent, ability, targets)
  target = targets:first()
  if not target:is_valid() then return end
  
  target = targets:first()
  
  hit = parent:special_attack(target, "Reflex", "Ranged")
  amount = -2000
  if hit:is_miss() then
    return
  elseif hit:is_graze() then
    amount = amount / 2
  elseif hit:is_hit() then
    -- do nothing
  elseif hit:is_crit() then
    amount = amount * 2
  end
  
  target:change_overflow_ap(amount)
end