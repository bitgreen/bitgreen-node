
variable "hcloud_token" {
  type = string
}

variable "pubkey" {
  type = string
}

variable "privatekey" {
  type = string
}

module "kube-hetzner" {
  providers = {
    hcloud = hcloud
  }
  hcloud_token = var.hcloud_token
  ssh_public_key = var.pubkey
  ssh_private_key = var.privatekey
  network_region = "eu-central" # change to `us-east` if location is ash

  control_plane_nodepools = [
    {
      name        = "control-plane-fsn1",
      server_type = "cpx11",
      location    = "fsn1",
      labels      = [],
      taints      = [],
      count       = 1
    },
    {
      name        = "control-plane-nbg1",
      server_type = "cpx11",
      location    = "nbg1",
      labels      = [],
      taints      = [],
      count       = 1
    },
    {
      name        = "control-plane-hel1",
      server_type = "cpx11",
      location    = "hel1",
      labels      = [],
      taints      = [],
      count       = 1
    }
  ]

  agent_nodepools = [
    {
      name        = "agent-small",
      server_type = "cpx11",
      location    = "fsn1",
      labels      = [],
      taints      = [],
      count       = 1
    },
    {
      name        = "agent-large",
      server_type = "cpx21",
      location    = "nbg1",
      labels      = [],
      taints      = [],
      count       = 1
    },
    {
      name        = "storage",
      server_type = "cpx21",
      location    = "fsn1",
      # Fully optional, just a demo
      labels = [
        "node.kubernetes.io/server-usage=storage"
      ],
      taints = [
        "server-usage=storage:NoSchedule"
      ],
      count = 1
    }
  ]

  load_balancer_type     = "lb11"
  load_balancer_location = "fsn1"

  cluster_name = "bitgreen"

  # Whether to use the cluster name in the node name, in the form of {cluster_name}-{nodepool_name}, the default is "true".
  # use_cluster_name_in_node_name = false

  # Adding extra firewall rules, like opening a port
  # More info on the format here https://registry.terraform.io/providers/hetznercloud/hcloud/latest/docs/resources/firewall
  # extra_firewall_rules = [
  #   # For Postgres
  #   {
  #     direction       = "in"
  #     protocol        = "tcp"
  #     port            = "5432"
  #     source_ips      = ["0.0.0.0/0", "::/0"]
  #     destination_ips = [] # Won't be used for this rule 
  #   },
  #   # To Allow ArgoCD access to resources via SSH
  #   {
  #     direction       = "out"
  #     protocol        = "tcp"
  #     port            = "22"
  #     source_ips      = [] # Won't be used for this rule 
  #     destination_ips = ["0.0.0.0/0", "::/0"]
  #   }
  # ]

  # If you want to configure a different CNI for k3s, use this flag
  # possible values: flannel (Default), calico, and cilium
  # CAVEATS: Calico is not supported when not using the Hetzner LB (like when enable_klipper_metal_lb is set to true or when using a single node cluster),
  # because of the following issue https://github.com/k3s-io/klipper-lb/issues/6.
  # As for Cilium, we allow infinite configurations, please check the CNI section of the readme over at https://github.com/kube-hetzner/terraform-hcloud-kube-hetzner/#cni.
  # cni_plugin = "cilium"

  # If you want to disable the k3s default network policy controller, use this flag!
  # Both Calico and Ciliun cni_plugin values override this value to true automatically, the default is "false".
  # disable_network_policy = true

  # If you want to disable the automatic use of placement group "spread". See https://docs.hetzner.com/cloud/placement-groups/overview/
  # That may be useful if you need to deploy more than 500 nodes! The default is "false".
  # placement_group_disable = true

  # By default, we allow ICMP ping in to the nodes, to check for liveness for instance. If you do not want to allow that, you can. Just set this flag to true (false by default).
  # block_icmp_ping_in = true

  # You can enable cert-manager (installed by Helm behind the scenes) with the following flag, the default is "false".
  # enable_cert_manager = true

  # IP Addresses to use for the DNS Servers, set to an empty list to use the ones provided by Hetzner, defaults to ["1.1.1.1", " 1.0.0.1", "8.8.8.8"].
  # For rancher installs, best to leave it as default.
  # dns_servers = []

  # When this is enabled, rather than the first node, all external traffic will be routed via a control-plane loadbalancer, allowing for high availability.
  # The default is false.
  # use_control_plane_lb = true

  # You can enable Rancher (installed by Helm behind the scenes) with the following flag, the default is "false".
  # When Rancher is enabled, it automatically installs cert-manager too, and it uses rancher's own self-signed certificates.
  # See for options https://rancher.com/docs/rancher/v2.0-v2.4/en/installation/resources/advanced/helm2/helm-rancher/#choose-your-ssl-configuration
  # The easiest thing is to leave everything as is (using the default rancher self-signed certificate) and put Cloudflare in front of it.
  # As for the number of replicas, by default it is set to the numbe of control plane nodes.
  # You can customized all of the above by adding a rancher_values.yaml file at the root of your module, which is just a helm values file. 
  # See the rancher_values.yaml.example file located at the root of the project.
  # After the cluster is deployed, you can always use HelmChartConfig definition to tweak the configuration.
  # IMPORTANT: Rancher's install is quite memory intensive, you will require at least 4GB if RAM, meaning cx21 server type (for your control plane).
  # ALSO, in order for Rancher to successfully deploy, you have to set the "rancher_hostname".
  # enable_rancher = true

  # If using Rancher you can set the Rancher hostname, it must be unique hostname even if you do not use it.
  # If not pointing the DNS, you can just port-forward locally via kubectl to get access to the dashboard.
  # rancher_hostname = "rancher.xyz.dev"

  # When Rancher is deployed, by default is uses the "latest" channel. But this can be customized.
  # The allowed values are "stable" or "latest".
  # rancher_install_channel = "stable"

  # Finally, you can specify a bootstrap-password for your rancher instance. Minimum 48 characters long!
  # If you leave empty, one will be generated for you.
  # (Can be used by another rancher2 provider to continue setup of rancher outside this module.)
  # rancher_bootstrap_password = ""

  # Separate from the above Rancher config (only use one or the other). You can import this cluster directly on an
  # an already active Rancher install. By clicking "import cluster" choosing "generic", giving it a name and pasting
  # the cluster registration url below. However, you can also ignore that and apply the url via kubectl as instructed
  # by Rancher in the wizard, and that would register your cluster too.
  # More information about the registration can be found here https://rancher.com/docs/rancher/v2.6/en/cluster-provisioning/registered-clusters/
  # rancher_registration_manifest_url = "https://rancher.xyz.dev/v3/import/xxxxxxxxxxxxxxxxxxYYYYYYYYYYYYYYYYYYYzzzzzzzzzzzzzzzzzzzzz.yaml"


  # Extra values that will be passed to the `extra-manifests/kustomization.yaml.tpl` if its present.
  # extra_kustomize_parameters={}
}

provider "hcloud" {
  token = var.hcloud_token
}

terraform {
  required_version = ">= 1.2.0"
  required_providers {
    hcloud = {
      source  = "hetznercloud/hcloud"
      version = ">= 1.35.1"
    }
  }
}
