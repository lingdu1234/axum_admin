import{s as e}from"./index-174ad176.js";import{k as t,A as u}from"./index.js";import{s as l}from"./constDynamicRoutes-5c9c22de.js";import{_ as a}from"./user-manage-form.vue_vue_type_script_setup_true_lang-fa104124.js";import"./pagination-87b07e34.js";import"./_plugin-vue_export-helper-0909ef10.js";import"./right-tool-bar-f10e816b.js";import"./usePermission-16d8d5ac.js";import"./useTableUtil-0e4d2f9a.js";import"./dict-2940434e.js";import"./user-manage-dialog.vue_vue_type_script_setup_true_lang-acbceb98.js";import"./useFormUtil-98c17c20.js";const r=Vue.defineComponent({name:l.userManage.path}),n=Vue.defineComponent({...r,setup(l){const{t:r}=VueI18n.useI18n({useScope:"global"}),n={label:"dept_name",children:"children"},o=Vue.ref(),d=Vue.ref(),s=Vue.ref(),p=Vue.ref([]),i=e=>{d.value=e.dept_id},c=(e,t)=>!e||t.dept_name.includes(e);return Vue.watch(o,(e=>{s.value.filter(e)})),(async()=>{const{data:e,execute:l}=t(u.getDeptTree);await l(),p.value=e.value})(),Vue.provide("deptTree",p),(t,u)=>(Vue.openBlock(),Vue.createElementBlock("div",null,[Vue.createVNode(Vue.unref(ElementPlus.ElRow),{gutter:20},{default:Vue.withCtx((()=>[Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:4,xs:24},{default:Vue.withCtx((()=>[Vue.createElementVNode("div",null,[Vue.createVNode(Vue.unref(ElementPlus.ElInput),{modelValue:o.value,"onUpdate:modelValue":u[0]||(u[0]=e=>o.value=e),placeholder:Vue.unref(r)("dept.searchTip"),clearable:"","prefix-icon":Vue.unref(e),class:"m-b-20px"},null,8,["modelValue","placeholder","prefix-icon"])]),Vue.createElementVNode("div",null,[Vue.createVNode(Vue.unref(ElementPlus.ElTree),{ref_key:"deptTreeRef",ref:s,data:p.value,props:n,"expand-on-click-node":!1,"filter-node-method":c,"default-expand-all":"",onNodeClick:i},null,8,["data"])])])),_:1}),Vue.createVNode(Vue.unref(ElementPlus.ElCol),{span:20,xs:24},{default:Vue.withCtx((()=>[Vue.createVNode(a,{dept_id:d.value},null,8,["dept_id"])])),_:1})])),_:1})]))}});export{n as default};